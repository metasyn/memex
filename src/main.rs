extern crate clap;
use clap::{App, Arg};

extern crate colored;
use colored::*;

use std::collections::HashSet;
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{BufReader, Read, Result};
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use regex::Regex;


///////////
// UTILS //
///////////

fn info<T>(msg: T)
where
    T: AsRef<str> + Display,
{
    println!("{} {}", "[INFO]".bright_cyan().bold(), msg)
}

fn err<T>(msg: T)
where
    T: AsRef<str> + Display,
{
    println!("{} {}", "[ERROR]".bright_red().bold(), msg)
}

// TODO: implement clean

/////////////
// STRUCTS //
/////////////

struct Entry {
    id: OsString,
    content: String,
    references: Vec<String>,
}

////////////
// REGEX //
///////////

lazy_static! {static ref INTERNAL_LINK_REGEX: Regex = Regex::new("\\[\\[(.+?)]]").unwrap();}


/////////////
// EXTRACT //
/////////////

fn extract_content_from_path(path_buf: &PathBuf) -> Option<(OsString, String)> {
    // check file stem
    let stem = path_buf.file_stem();
    if stem.is_none() {
        err(format!("{:?} has no file stem...", path_buf));
        return None;

    }

    // check loading content
    let content = load_file(path_buf);
    if content.is_err() {
        err(format!("{:?} content could not be loaded...", path_buf));
        return None
    }
    return Some((OsString::from(stem.unwrap()), content.unwrap()));

}

fn extract_references_from_content(body: &str) -> Vec<String> {
    INTERNAL_LINK_REGEX
        .captures_iter(body)
        .filter_map(|c| c.get(1))
        .map(|x| String::from(x.as_str()))
        .collect()
}

// recursively find all .md files in the content path
// and turn them into entries
fn collect_entries(content_path: &str) -> Result<Vec<Entry>> {
    let mut result: Vec<Entry> = Vec::new();
    let mut todo: Vec<PathBuf> = Vec::new();
    let root = Path::new(content_path).to_path_buf();
    todo.push(root);

    while todo.len() > 0 {
        let file = todo.pop().unwrap();
        let entries = fs::read_dir(file)
            .expect("unable to read file")
            .map(|e| e.map(|x| x.path()))
            .collect::<Result<HashSet<PathBuf>>>()
            .expect("unable to collect entries.");

        let directories = entries
            .iter()
            .filter(|e| e.is_dir())
            .map(|e| e.to_path_buf())
            .collect::<HashSet<PathBuf>>();

        let files: Vec<Entry> = entries
            .difference(&directories)
            .filter(|e| match e.extension() {
                Some(ext) => ext == "md",
                None => false
            })
            .filter_map(|e| extract_content_from_path(e))
            .map(|(stem, content)|
                Entry{
                    id: OsString::from(stem),
                    content: content.clone(),
                    references: extract_references_from_content(content.as_str()),
                }
            )
            .collect();

        todo.extend(directories.into_iter().collect::<Vec<PathBuf>>());
        result.extend(files);
    }
    return Ok(result);
}

//////////////
// COMMANDS //
//////////////

// TODO: pass template path
fn build(content_path: &str) -> Result<()> {
    info("building memex...");

    match collect_entries(content_path) {
        Ok(entries) => {
            info(
                format!(
                    "compiling {} entries...",
                    entries.len().to_string().bright_yellow()
                )
            );
            // handle specific files here
            let base_template = load_file("./templates/base.html")?;

            for entry in entries{
                // TODO: keep track of references before doing replacements
                // TODO: also calculate directory
                // finish replacements
                // write files
                let contents = entry.content.as_str();
                let references = format_references(entry.references);
                let replacements = vec![("content", contents), ("references", &references)];
                let replaced = replace_templates(base_template.clone(), replacements);
                info(replaced);
            }
        }
        Err(_) => err("couldn't collect paths..."),
    }
    Ok(())
}


fn load_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn format_references(references: Vec<String>) -> String {
    references
        .iter()
        .map(|x| format!("[[{}]]", x))
        .collect::<Vec<String>>()
        .join(" ")
}

///////////////
// TEMPLATES //
///////////////

fn make_template(item: &str) -> String {
    return format!("{{{{ {} }}}}", item);
}

fn replace_template<'a>(body: String, item: &str, replacement: &str) -> String {
    return body.replace(make_template(item).as_str(), replacement);
}

fn replace_templates<'a>(mut body: String, mapping: Vec<(&str, &str)>) -> String {
    for (key, value) in mapping.iter() {
        body = replace_template(body, key, value)
    }
    return body;
}

//////////
// MAIN //
//////////

fn main() -> Result<()> {
    let matches = App::new("memex")
        .version("0.2")
        .author("xander johnson xander@metasyn.pw")
        .about("personal memex")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("sets the level of verbosity."),
        )
        .arg(
            Arg::with_name("content")
                .short("c")
                .help("sets the content folder"),
        )
        .subcommand(App::new("build").about("builds the memex"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("build") {
        let content_path = matches.value_of("content").unwrap_or("content/entries");
        return build(content_path);
    }

    Ok(())
}

///////////
// TESTS //
///////////

#[cfg(test)]
mod tests {
    // import namespace above here
    use super::*;

    #[test]
    fn test_make_template() {
        assert_eq!(make_template("test"), "{{ test }}")
    }

    #[test]
    fn test_replace_template() {
        assert_eq!(
            replace_template(String::from("{{ test }}"), "test", "fab"),
            "fab"
        )
    }

    #[test]
    fn test_replace_templates() {
        assert_eq!(
            replace_templates(
                String::from("{{ test }} {{ something }}"),
                vec![("test", "fab"), ("something", "replacement")]
            ),
            "fab replacement"
        )
    }

    #[test]
    fn test_extract_content_from_path() {
        let path_buf = PathBuf::from("Cargo.toml");
        let res = extract_content_from_path(&path_buf);
        assert!(res.is_some());
        assert!(res.unwrap().0 == "Cargo");
    }


    #[test]
    fn test_extract_references_from_content() {
        let res = extract_references_from_content("testing [[one]] [[two]]");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "one");
        assert_eq!(res[1], "two");

    }
}
