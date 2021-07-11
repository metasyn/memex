extern crate clap;
use clap::{App, Arg};

extern crate colored;
use colored::*;

use std::collections::HashSet;
use std::io::Result;
use std::fs;
use std::path::{Path, PathBuf};


fn info(msg: &str) {
    println!("{} {}", "[INFO]".bright_cyan().bold(), msg)
}

fn err(msg: &str) {
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

fn build(content_path: &str) -> Result<()> {
    info("building memex...");
    match collect_entries(content_path) {
        Ok(paths) => {
            info(format!("compiling {} entries...", paths.len().to_string().bright_yellow()).as_str())
        },
        Err(_) => err("couldn't collect paths..."),
    }
    err("hey");
    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("memex")
        .version("0.2")
        .author("xander johnson xander@metasyn.pw")
        .about("personal memex")
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("sets the level of verbosity."))
        .arg(Arg::with_name("content")
             .short("c")
             .help("sets the content folder"))
        .subcommand(App::new("build").about("builds the memex"))
        .get_matches();


    if let Some(ref matches) = matches.subcommand_matches("build") {
        let content_path = matches.value_of("content").unwrap_or("content/entries");
        return build(content_path);
    }

    Ok(())
}
