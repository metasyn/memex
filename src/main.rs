extern crate clap;
use clap::{App, Arg};

extern crate colored;
use colored::*;

use std::collections::HashSet;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{BufReader, Read, Result};
use std::path::{Path, PathBuf};

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

// recursively find all .md files in the content path
fn collect_entries(content_path: &str) -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
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

        let files: Vec<_> = entries
            .difference(&directories)
            .filter(|e| e.extension().unwrap() == "md")
            .map(|e| e.to_path_buf())
            .collect();

        todo.extend(directories.into_iter().collect::<Vec<PathBuf>>());
        result.extend(files);
    }
    return Ok(result);
}

// TODO: implement clean

// TODO: pass template path
fn build(content_path: &str) -> Result<()> {
    info("building memex...");

    match collect_entries(content_path) {
        Ok(paths) => {
            info(
                format!(
                    "compiling {} entries...",
                    paths.len().to_string().bright_yellow()
                )
                .as_str(),
            );
            // handle specific files here
            let base_template = load_file("./templates/base.html")?;
            //
            for path in paths {
                // TODO: keep track of references before doing replacements
                // TODO: also calculate directory
                // finish replacements
                // write files
                let contents = load_file(path)?;

                let replacements = vec![("directory", "something"), ("content", contents.as_str())];
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
}
