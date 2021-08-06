use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{self, DirEntry, File, create_dir_all, remove_dir_all};
use std::io::prelude::Write;
use std::io::{BufReader, Error, ErrorKind, Read, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use color_eyre::{Report};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use clap::{App, Arg};
use colored::*;
use chrono::{Local, NaiveDate};
use notify::{RecommendedWatcher, RecursiveMode, Watcher}; use dither::prelude::*;
use rss::Channel;

use rss::validation::Validate;

/////////////
// STRUCTS //
/////////////

#[derive(Debug, Clone)]
struct Entry {
    id: String,
    content: String,
    modification_date: String,
    links: Vec<String>,
    references: Option<Vec<String>>,
    path: PathBuf,
}

#[derive(Debug, Default)]
struct DirectoryItem {
    idx: usize,
    val: String,
    children: Vec<usize>,
}

impl DirectoryItem {
    fn new(idx: usize, val: String) -> Self {
        Self {
            idx,
            val,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct DirectoryTree {
    arena: Vec<DirectoryItem>,
}

impl DirectoryTree {
    fn node(&mut self, val: String) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(DirectoryItem::new(idx, val));
        idx
    }
}

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

fn load_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    return Ok(contents);
}

// see https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

// set the comrak html -> md settings
fn comrak_options() -> comrak::ComrakOptions {
    let mut options = comrak::ComrakOptions::default();
    options.render.unsafe_ = true;
    options.render.escape = false;

    options.parse.smart = true;

    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tagfilter = false;
    return options;
}

// pretty much always want this backtrace on
fn setup() -> std::result::Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    Ok(())
}

////////////
// REGEX //
///////////

lazy_static! {
    static ref COMRAK_OPTIONS: comrak::ComrakOptions = comrak_options();
}

lazy_static! {
    static ref INTERNAL_LINK_REGEX: Regex = Regex::new("\\[\\[(?P<link>.+?)]]").unwrap();
}
lazy_static! {
    static ref HEADER_REGEX: Regex = Regex::new(r"^\s*(?P<level>#+)\s*(?P<heading>.*)").unwrap();
}
lazy_static! {
    static ref NON_WORD_REGEX: Regex = Regex::new(r"[^\w-]+").unwrap();
}
lazy_static! {
    static ref DITHERED_IMG_REGEX: Regex = Regex::new(r"(?P<img><img src=.*?resources/img/dithered_(?P<name>.+?)\..+?>)").unwrap();
}

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
        return None;
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

    let fallback = Local::today().naive_local().to_string();
    let dt: NaiveDate = match output {
        Ok(res) => {
            let date = String::from_utf8(res.stdout).expect("unable to read git log output");

            let clean = date.replace("format:", "").replace("\"", "");
            let subset = clean.trim();

            if subset.is_empty() {
                return fallback;
            }

            let date_time = NaiveDate::parse_from_str(subset, "%Y-%m-%d %H:%M:%S %z");
            match date_time {
                Ok(dt) => dt,
                Err(e) => panic!("subset: {} \n {}", subset, e),
            }
        }
        Err(_) => {
            // Could check modification time. But I check everything into git...
            return fallback;
        }
    };

    return dt.to_string().replace("-", ".");
}

fn extract_references<'a>(entries: &'a Vec<Entry>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();
    for entry in entries {
        for link in &entry.links {
            result
                .entry(link.clone())
                .or_insert(Vec::new())
                .push(entry.id.clone());
        }
    }
    return result;
}

fn extract_outline_md(content: &str) -> String {
    let mut result = String::from("");
    if HEADER_REGEX.is_match(content) {
        let headers = content
            .split("\n")
            .filter(|x| x.starts_with("#"))
            .collect::<Vec<&str>>();

        for line in headers {
            // calculate indent before cleaning
            let indent = "  ".repeat(line.matches("#").count());
            result.push_str(&indent);

            // replace anything that isn't a word char
            let header_link = make_header_link(line);
            let heading = HEADER_REGEX
                .captures(line)
                .expect("invalid regex; no captures for header.")
                .name("heading")
                .expect("invalid regex; no capture group named heading")
                .as_str();

            let link = format!(
                "* <a class='header' href='#{}'>{}</a>\n",
                header_link, heading
            );
            result.push_str(&link);
        }
    }
    return result;
}

fn extract_directory(entries: &Vec<Entry>, root_name: &str) -> DirectoryTree {
    let mut paths = entries
        .iter()
        .map(|x| {
            x.path
                .as_path()
                .components()
                .map(|x| x.as_os_str().to_str().unwrap().replace(".md", ""))
                .filter(|x| x != "404")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    paths.sort();

    let mut tree: DirectoryTree = DirectoryTree::default();
    let directory = tree.node(root_name.to_string());
    let mut base = directory;

    for path_segments in paths {
        for (idx, segment) in path_segments.iter().enumerate() {
            // always reset the base to the root on the first segment
            if idx == 0 {
                base = directory;
            }

            // fetch (and create if missing)
            let node = tree.node(segment.clone());

            // add nodes
            if !tree.arena[base].children.contains(&node) {
                tree.arena[base].children.push(node);
            }
            // switch base
            base = node;
        }
    }

    return tree;
}

fn extract_recent_entries(entries: &mut Vec<Entry>) -> String {
    let length = min(entries.len(), 20);

   entries
       .sort_by(|a, b|  b.modification_date.partial_cmp(&a.modification_date).unwrap());

    entries
       .truncate(length);

    return entries
       .iter()
       .map(|x| format!("* {} [[{}]]", x.modification_date, x.id))
       .collect::<Vec<String>>()
       .join("\n");


}


////////////
// FORMAT //
////////////

fn format_directory(tree: &DirectoryTree) -> String {
    fn traverse(tree: &DirectoryTree, item: &DirectoryItem, res: &mut String, depth: u8) {
        if item.children.len() > 0 {
            res.push_str(
                format!(
                    "<details style=\"--depth: {}\"><summary>{}</summary>\n",
                    depth, item.val
                )
                .as_str(),
            );

            for child in &item.children {
                traverse(tree, &tree.arena[*child], res, depth + 1)
            }

            res.push_str("</details>\n");
        } else {
            res.push_str(format!("* [[{}]]\n", item.val).as_str());
        }
    }

    // own the string here, so we can add to it
    let mut formatted = String::new();
    // update the string recrusively
    traverse(tree, &tree.arena[0], &mut formatted, 0);

    return formatted;
}

fn format_directory_page(tree: &DirectoryTree) -> String {
    fn traverse(tree: &DirectoryTree, item: &DirectoryItem, res: &mut String, depth: u8) {

        let indent = "  ".repeat(depth.into());

        if item.children.len() > 0 {
            res.push_str(
                format!(
                    "{}* {}\n",
                    indent, item.val,
                    ).as_str()
                );

            for child in &item.children {
                traverse(tree, &tree.arena[*child], res, depth + 1)
            }
        } else {
            res.push_str(format!("{}* [[{}]]\n", indent, item.val).as_str());
        }
    }

    // own the string here, so we can add to it
    let mut formatted = String::new();
    // update the string recrusively
    traverse(tree, &tree.arena[0], &mut formatted, 0);

    return formatted;
}

fn format_references(references: Vec<String>) -> String {
    references
        .iter()
        .map(|x| format!("[[{}]]", x))
        .collect::<Vec<String>>()
        .join(" ")
}

fn format_img_dither_wrap_anchor(body: &str) -> String {
    return DITHERED_IMG_REGEX
        .replace_all(body, |cap: &Captures| {
            let img = &cap[1];
            let name = &cap[2];

            return format!(
                "<a class='img' href=\"resources/img/{}.png\">{}</a>",
                name, img,
            )
        })
        .to_string();
}

/////////////
// CONVERT //
/////////////

fn convert_internal_to_md(content: &str) -> String {
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
                None => false,
            })
            .filter_map(|e| extract_content_from_path(e))
            .map(|(stem, content, path)| Entry {
                id: String::from(stem.to_str().unwrap()),
                content: content.clone(),
                links: extract_links_from_content(content.as_str()),
                modification_date: extract_modification_date_from_path(path),
                path: path.strip_prefix(content_path).unwrap().to_path_buf(),
                references: None,
            })
            .collect();

        todo.extend(directories.into_iter().collect::<Vec<PathBuf>>());
        result.extend(files);
    }

    // now that all entries are collected, we can calculate references
    let references = extract_references(&result);

    // attach references
    result.iter_mut().for_each(|x| {
        let opt = references.get(&x.id);
        x.references = match &opt {
            Some(refs) => Some(refs.to_vec()),
            None => None,
        };
    });

    return Ok(result);
}

///////////////
// TEMPLATES //
///////////////

fn render_md(s: &str) -> String {
    // TODO: figure out why this didn't work with replace_all on the entire string
    // it seems weird to have to split the string first, then rejoin it.
    // whatever. this is for making the header links work in the outline.
    let prerender = s
        .split("\n")
        .map(|x| {
            HEADER_REGEX
                .replace_all(&x, |caps: &Captures| {
                    format!(
                        "{} <a name='{}'>{}</a>",
                        &caps[1],
                        make_header_link(&caps[2]),
                        &caps[2]
                    )
                })
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");

    let wrapped = format_img_dither_wrap_anchor(prerender.as_str());
    return comrak::markdown_to_html(&wrapped, &COMRAK_OPTIONS);
}

fn make_template(item: &str) -> String {
    return format!("{{{{ {} }}}}", item);
}

fn replace_template<'a>(body: String, item: &str, replacement: &str, line_by_line: bool) -> String {
    let repl = convert_internal_to_md(replacement);
    let html = match line_by_line {
        true => repl
            .split("\n")
            .map(|x| render_md(x))
            .collect::<Vec<String>>()
            .join("\n"),
        false => render_md(&repl),
    };
    return body.replace(make_template(item).as_str(), &html);
}

fn replace_templates<'a>(mut body: String, mapping: Vec<(&str, &str, bool)>) -> String {
    for (key, value, line_by_line) in mapping.iter() {
        body = replace_template(body, key, &value, *line_by_line)
    }
    return String::from(body);
}

fn make_header_link(header: &str) -> String {
    let temp = HEADER_REGEX
        .replace_all(header, "$heading")
        // these should just disappear
        .replace("'", "")
        .replace("\"", "");

    // everthing else not a word becomes a dash
    let clean = NON_WORD_REGEX.replace_all(&temp.trim(), "-");
    return String::from(clean);
}



//////////////
// COMMANDS //
//////////////

fn build(content_path: &str, template_path: &str, destination_path: &str, resources_path: &str) -> Result<()> {
    hey("building memex...");

    let dest_path = Path::new(destination_path);

    hey("cleaning destination dir...");
    let removal = remove_dir_all(destination_path);
    if removal.is_err() {
        hey("output destination doesn't exist...");
    }
    create_dir_all(destination_path)?;

    hey("validating rss...");
    let rss_path = Path::new("rss.xml").to_path_buf();
    let _channel = parse_rss(&rss_path);
    fs::copy(rss_path, dest_path.join("rss.xml"))?;


    hey("copying resources...");
    let dest_path = Path::new(destination_path);
    copy_dir_all(resources_path, dest_path.join(resources_path))?;

    return match collect_entries(content_path) {
        Ok(mut entries) => {
            hey(format!(
                "compiling {} entries...",
                entries.len().to_string().bright_yellow()
            ));
            let base_template = load_file(template_path)?;

            let directory = extract_directory(&entries, "pages");
            let formatted_directory = format_directory(&directory);
            let recents = extract_recent_entries(&mut entries.clone());

            // handle special case for directory page
            entries
                .iter_mut()
                .filter(|x| x.id == "directory")
                .for_each(|x| x.content = format_directory_page(&directory));


            for entry in entries {
                // get or set replacements
                let contents = entry.content.as_str();
                let timestamp = entry.modification_date.to_string();
                let references = format_references(entry.references.unwrap_or(Vec::new()));
                let outline = extract_outline_md(contents);

                // make replacements
                let replacements = vec![
                    ("directory", formatted_directory.as_str(), true),
                    ("content", contents, false),
                    ("references", &references, false),
                    ("timestamp", timestamp.as_str(), false),
                    ("toc", &outline, false),
                    ("recent", recents.as_str(), false),
                ];
                let html = replace_templates(base_template.clone(), replacements);

                // write replacements
                let fname = format!("{}/{}.html", destination_path, entry.id);
                hey(format!("{} => {}", entry.id.yellow(), fname.green()));
                let mut fd = File::create(fname)?;
                fd.write_all(html.as_bytes())?
            }
            return Ok(());
        }
        Err(e) => {
            nope("couldn't collect paths...");
            Err(e)
        }
    };
}

fn parse_rss(input_rss_path: &PathBuf) -> Result<Channel> {
    let file = File::open(input_rss_path).unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    channel.validate()
        .expect("should be able to validate rss.xml file...");

    return Ok(channel)
}



fn watch(content_path: &str, template_path: &str, destination_path: &str, resources_path: &str) -> notify::Result<()> {
   // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(content_path, RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.

    let watching = "watching for changes!".bright_magenta().to_string();
    hey(&watching);
    loop {
        match rx.recv() {
            Ok(event) => {
                hey("updating...");
                println!("{:#?}", event);
                build(content_path, template_path, destination_path, resources_path)?;
                hey(&watching);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}


fn rename(content_path: &str, old: &str, new: &str) -> Result<()> {
    let entries = collect_entries(content_path)
        .expect("couldn't collect entries.");
    hey(format!("replacing {} with {}", old, new));

    let as_string = format!(r"\[\[{}\]\]", old);
    let regex = Regex::new(as_string.as_str())
        .expect("invalid replacement regex");


    for entry in entries {
        if regex.is_match(&entry.content) {
            let replaced = regex.replace_all(&entry.content, format!("[[{}]]", new));
            let path = Path::new(content_path).join(entry.path);
            hey(format!("writing to {:#?}", path.as_os_str()));
            let mut fd = File::create(path)
                .expect("couldn't create new file");
            fd.write_all(replaced.as_bytes())
                .expect("couldn't write new file");
        }
    }

    return Ok(());
}

fn scratch() {
    // 0 2
    // 3 1
    let ditherer = dither::ditherer::Ditherer::new(
        9.,
        &[
            // dx, dy, mul
            (1, 0, 2.),
            (-1, 1, 1.),
            (-1, 0, 3.),
        ],
    );

    let ditherer = dither::ditherer::Ditherer::new(
        16.,
        &[
            // dx, dy, mul
            (0, -1, 0.),
            (0, -2, 12.),
            (0, -3, 3.),
            (0, -4, 15.),
            //
            (1, -1, 8.),
            (1, -2, 4.),
            (1, -3, 11.),
            (1, -4, 7.),
            //
            (2, -1, 2.),
            (2, -2, 14.),
            (2, -3, 1.),
            (2, -4, 13.),
            //
            (3, -1, 10.),
            (3, -2, 6.),
            (3, -3, 9.),
            (3, -4, 5.),
        ],
    );

}

fn dither(resouces_path: &str, destination_path: &str) {
    let path = Path::new(resouces_path).join("img/");
    let entries = fs::read_dir(path)
        .expect("couldn't read destination")
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.path().to_str().unwrap().contains("maranta"))
        .collect::<Vec<DirEntry>>();

    // 0 2
    // 3 1
    let ditherer = dither::ditherer::Ditherer::new(
        4.,
        &[
            (0, 0, 0.),
            (0, 1, 3.),
            (1, 0, 2.),
            (1, 1, 1.),
        ],
    );


    let quantize = &dither::create_quantize_n_bits_func(4)
        .expect("couldn't create quantizer");

    for entry in entries {
        let path = entry.path().to_path_buf();

        let img: Img<RGB<f64>> = Img::<RGB<f64>>::load(path.as_path())
            .expect("couldn't load image");

        let output_img = ditherer
            .dither(img, RGB::map_across(quantize))
            .convert_with(|rgb| rgb.convert_with(clamp_f64_to_u8));

        let output_path = Path::new("test").join(path.clone()).to_path_buf();
        println!("{:?} => {:?}", entry, output_path.as_os_str());

        create_dir_all("test/resources/img").unwrap();
        output_img.save(output_path.as_path()).unwrap();

    }
}

// TODO: implement build rss
// TODO: add native dithering approach

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
                .help("sets the level of verbosity.")
        )
        .arg(
            Arg::with_name("content")
                .short("c")
                .long("content")
                .help("sets the content folder")
        )
        .arg(
            Arg::with_name("template")
                .short("t")
                .long("template")
                .help("sets the template")
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("destination")
                .help("sets the destination directory")
        )
        .arg(
            Arg::with_name("resources")
                .short("r")
                .long("resources")
                .help("sets the resources directory")
        )
        .subcommand(App::new("build").about("builds the memex"))
        .subcommand(App::new("watch").about("watches for file system changes and builds the memex on each change"))
        .subcommand(App::new("dither").about("dithers images in dist"))
        .subcommand(App::new("rename").about("updates internal page id across entries")
            .arg(
                Arg::with_name("old")
                    .required(true)
                    .long("old")
                    .short("o")
                    .help("old name")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("new")
                    .required(true)
                    .long("new")
                    .short("n")
                    .help("new name")
                    .takes_value(true)
            ))
        .get_matches();

    setup().expect("couldn't setup.");

    let content_path = matches.value_of("content").unwrap_or("content/entries");
    let template_path = matches.value_of("template").unwrap_or("templates/base.html");
    let destination_path = matches.value_of("destination").unwrap_or("dist");
    let resources_path = matches.value_of("resources").unwrap_or("resources");

    if matches.subcommand_matches("build").is_some() {
        let b = build(content_path, template_path, destination_path, resources_path);

        if b.is_ok() {
            hey("✨ Done!".bright_cyan().to_string());
        }

        return b;
    }

    if matches.subcommand_matches("watch").is_some() {
        let res = watch(content_path, template_path, destination_path, resources_path);
        if res.is_err() {
            let msg =  format!("error watching: {}", res.unwrap_err().to_string());
            return Err(Error::new(ErrorKind::Other, msg));
        }
    }

    if let Some(matches) = matches.subcommand_matches("rename") {
        let old = matches.value_of("old")
            .expect("no old value provided");
        let new = matches.value_of("new")
            .expect("no new value provided");
        return rename(content_path, old, new)
    }

    if matches.subcommand_matches("dither").is_some() {
        dither(resources_path, destination_path);
        return Ok(())
    }

    let msg =  "invalid command";
    return Err(Error::new(ErrorKind::Other, msg));
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
            replace_template(String::from("{{ test }}"), "test", "fab", false),
            "<p>fab</p>\n"
        )
    }

    #[test]
    fn test_replace_templates() {
        assert_eq!(
            replace_templates(
                String::from("{{ test }} {{ something }}"),
                vec![("test", "fab", false), ("something", "replacement", false)],
            ),
            "<p>fab</p>\n <p>replacement</p>\n"
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
    fn test_convert_internal_to_md() {
        let converted = convert_internal_to_md("[[test]]");
        assert_eq!(converted, "[test](test.html)");
    }

    #[test]
    fn test_directory() {
        let entries = collect_entries("content/entries").unwrap();
        let directory = extract_directory(&entries, "pages");
        assert!(directory.arena.len() > 10);
    }
}
