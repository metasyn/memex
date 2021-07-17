extern crate clap;
use clap::{App, Arg};

extern crate colored;
use colored::*;

use std::process::Command;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{BufReader, Read, Result};
use std::io::prelude::Write;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use regex::Regex;

extern crate chrono;
use chrono::{Local, NaiveDate};

///////////
// UTILS //
///////////

fn hey<T>(msg: T)
where
    T: AsRef<str> + Display,
{
    println!("{} {}", "[INFO]".bright_cyan().bold(), msg)
}

fn nope<T>(msg: T)
where
    T: AsRef<str> + Display,
{
    println!("{} {}", "[ERROR]".bright_red().bold(), msg)
}

// TODO: implement clean

fn comrak_options() -> comrak::ComrakOptions {
    // i'm rendering my own content
    let mut options = comrak::ComrakOptions::default();
    options.render.unsafe_ = true;
    options.render.escape = false;

    options.parse.smart = true;

    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tagfilter = false;
    return options;
}



/////////////
// STRUCTS //
/////////////

// Entry struct is utility for referencing
// the id, content and back references of an entry
#[derive(Debug)]
struct Entry {
    id: String,
    content: String,
    modification_date: String,
    links: Vec<String>,
}

// Item struct is for calculating the directory
struct DirectoryItem {
    id: String,
    children: Vec<DirectoryItem>,
}


trait Builder {
    // collectors
    fn collect_entries(paths: &str) -> Result<Vec<Entry>>;

    // calculators
//    fn calculate_directory(entries: Vec<Entry>) -> DirectoryItem;
//    fn calculate_recent(entries: Vec<Entry>) -> Vec<Entry>;
//
//    // converters
//    fn convert_link_to_md(input: String) -> String;
//    fn convert_wrap_img_anchor(input: String) -> String;
//    fn convert_wrap_header_anarchor(input: String) -> String;
//    fn convert_md_to_html(input: String) -> String;
//
//
//    // render
//    fn render_outline(entry: Entry) -> String;
//    fn render_navigation(item: DirectoryItem) -> String;
}


////////////
// REGEX //
///////////

lazy_static! {static ref INTERNAL_LINK_REGEX: Regex = Regex::new("\\[\\[(?P<link>.+?)]]").unwrap();}
lazy_static! {static ref COMRAK_OPTIONS: comrak::ComrakOptions = comrak_options();}


/////////////
// EXTRACT //
/////////////

fn extract_content_from_path(path_buf: &PathBuf) -> Option<(OsString, String, &PathBuf)> {
    // check file stem
    let stem = path_buf.file_stem();
    if stem.is_none() {
        nope(format!("{:?} has no file stem...", path_buf));
        return None;

    }

    // check loading content
    let content = load_file(path_buf);
    if content.is_err() {
        nope(format!("{:?} content could not be loaded...", path_buf));
        return None
    }
    return Some((OsString::from(stem.unwrap()), content.unwrap(), path_buf));

}

fn extract_links_from_content(content: &str) -> Vec<String> {
    INTERNAL_LINK_REGEX
        .captures_iter(content)
        .filter_map(|c| c.get(1))
        .map(|x| String::from(x.as_str()))
        .collect()
}


fn extract_modification_date_from_path(path: &PathBuf) -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=\"format:%ci\"")
        .arg(path.as_os_str())
        .output();

    let dt: NaiveDate = match output {
        Ok(res) => {
            let date = String::from_utf8(res.stdout)
                .expect("unable to read git log output");

            let clean = date.replace("format:", "").replace("\"", "");
            let subset = clean.trim();

            let date_time = NaiveDate::parse_from_str(subset, "%Y-%m-%d %H:%M:%S %z");
            match date_time {
                Ok(dt) => dt,
                Err(e) => panic!("{}", e),
            }
        },
        Err(_) => {
            // Could check modification time. But I check everything into git...
            Local::today().naive_local()
        }
    };

    return dt.to_string().replace("-", ".");
}

/////////////
// CONVERT //
/////////////

fn convert_links_to_md(content: &str) -> String {
    return INTERNAL_LINK_REGEX
        .replace_all(&content, "[$link]($link.html)")
        .to_string();
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
            .map(|(stem, content, path)|
                Entry{
                    id: String::from(stem.to_str().unwrap()),
                    content: content.clone(),
                    links: extract_links_from_content(content.as_str()),
                    modification_date: extract_modification_date_from_path(path),
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
    hey("building memex...");

    match collect_entries(content_path) {
        Ok(entries) => {
            hey(
                format!(
                    "compiling {} entries...",
                    entries.len().to_string().bright_yellow()
                )
            );
            // handle specific files here
            let base_template = load_file("./templates/base.html")?;

            for entry in entries{

                // get or set replacements
                let contents = entry.content.as_str();
                let timestamp = entry.modification_date.to_string();

                // TODO: calculate references
                // TODO: calculate directory
                // TODO: calculate TOC
                let references = format_references(entry.links);

                // make replacements
                let replacements = vec![("content", contents), ("references", &references), ("timestamp", timestamp.as_str())];
                let html = render_templates(base_template.clone(), replacements);

                // write replacements
                let fname = format!("dist/{}.html", entry.id);
                hey(format!("{} => {}", entry.id.yellow(),  fname.green()));
                let mut fd = File::create(fname)?;
                fd.write_all(html.as_bytes())?
            }
        }
        Err(_) => nope("couldn't collect paths..."),
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

fn md(s: &str) -> String {
    return comrak::markdown_to_html(s, &COMRAK_OPTIONS);

}

fn make_template(item: &str) -> String {
    return format!("{{{{ {} }}}}", item);
}

fn render_template<'a>(body: String, item: &str, replacement: &str) -> String {
    return body.replace(make_template(item).as_str(), replacement);
}

fn render_templates<'a>(mut body: String, mapping: Vec<(&str, &str)>) -> String {


    // convert internal to md
    body = convert_links_to_md(body.as_str());

    for (key, value) in mapping.iter() {
        let val = md(value);
        body = render_template(body, key, &val)
    }
    return String::from(body);

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
    fn test_render_template() {
        assert_eq!(
            render_template(String::from("{{ test }}"), "test", "fab"),
            "fab"
        )
    }

    #[test]
    fn test_render_templates() {
        assert_eq!(
            render_templates(
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
        let res = extract_links_from_content("testing [[one]] [[two]]");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "one");
        assert_eq!(res[1], "two");
    }

    #[test]
    fn test_extract_modification_date_from_path() {
        let path = Path::new("Cargo.toml").to_path_buf();
        let res = extract_modification_date_from_path(&path).to_string();
        let regex = Regex::new(r"\d{4}.\d{2}.\d{2}").unwrap();
        let captures = regex.captures(res.as_str()).unwrap();
        assert_eq!(captures.len(), 1);
    }

    #[test]
    fn test_convert_links_to_md() {
        let converted = convert_links_to_md("[[test]]");
        assert_eq!(converted, "[test](test.html)");
    }
}
