use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{self, create_dir_all, remove_dir_all, File};
use std::io::prelude::Write;
use std::io::{BufReader, Error, ErrorKind, Read, Result, stdin};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use chrono::{Local, NaiveDate, Utc};
use clap::{App, Arg};
use color_eyre::Report;
use colored::*;
use lazy_static::lazy_static;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::{Captures, Regex};
use rss::{Channel, ItemBuilder, GuidBuilder};

use rss::validation::Validate;

static DOMAIN: &str = "metasyn.pw";

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

    static ref INTERNAL_LINK_REGEX: Regex = Regex::new("\\[\\[((?P<title>.*)\\|)?(?P<link>.+?)]]").unwrap();
    static ref HEADER_REGEX: Regex = Regex::new(r"^\s*(?P<level>#+)\s*(?P<heading>.*)").unwrap();
    static ref NON_WORD_REGEX: Regex = Regex::new(r"[^\w-]+").unwrap();
    static ref DITHERED_IMG_REGEX: Regex =
        Regex::new(r"(?P<img><img src=.*?resources/img/dithered_(?P<name>.+?)\..+?>)").unwrap();
    static ref MD_LINK_REGEX: Regex = Regex::new(r"\[(?P<title>.+?)\]\((?P<link>.+?)\)").unwrap();

    static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref TAG_SRC_REGEX: Regex = Regex::new("src=\"(?P<src>.+?)\"").unwrap();
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
        .filter_map(|c| c.get(3))
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

    entries.sort_by(|a, b| {
        b.modification_date
            .partial_cmp(&a.modification_date)
            .unwrap()
    });

    entries.truncate(length);

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

fn format_directory_page(existing_contents: &String, tree: &DirectoryTree) -> String {
    fn traverse(tree: &DirectoryTree, item: &DirectoryItem, res: &mut String, depth: u8) {
        let indent = "  ".repeat(depth.into());

        if item.children.len() > 0 {
            res.push_str(format!("{}* {}\n", indent, item.val,).as_str());

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

    return format!("{}\n{}", existing_contents, formatted);
}

fn format_references(references: Vec<String>) -> String {
    if references.is_empty() {
        return String::new();
    }

    let mut result = String::from("incoming links: ");
    result.push_str(
        references
            .iter()
            .filter(|x| *x != "meta")
            .map(|x| format!("[[{}]]", x))
            .collect::<Vec<String>>()
            .join(" ")
            .as_str(),
    );

    return result;
}

fn format_img_dither_wrap_anchor(body: &str) -> String {
    return DITHERED_IMG_REGEX
        .replace_all(body, |cap: &Captures| {
            let img = &cap[1];
            let name = &cap[2];

            return format!(
                "<a class='img' href=\"resources/img/{}.png\">{}</a>",
                name, img,
            );
        })
        .to_string();
}

/////////////
// CONVERT //
/////////////

fn convert_internal_to_md(content: &str) -> String {
    return INTERNAL_LINK_REGEX
        .replace_all(content, |caps: &Captures| {

            // must exist
            let link = &caps[3];

            let title = match caps.get(2) {
                Some(title) => title.as_str(),
                None => link,
            };

            format!(
                "[{}]({}.html)",
                title,
                link,

            )
        })
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

fn render_gemtext(s: &str) -> String {
    // first convert links
    let mut gem_links: Vec<String> = vec![
        format!("=> gemini://{}/index.gmi index and recent changes", DOMAIN).to_owned(),
        format!("=> gemini://{}/directory.gmi directory of all pages", DOMAIN).to_owned(),
    ];

    let mut http_links =  vec![];

    // Get all internal links
    let mut output = INTERNAL_LINK_REGEX
        .replace_all(s, |caps: &Captures| {
            let link = &caps[3];

            let gemlink = format!("=> gemini://{}/{}.gmi {}", DOMAIN, link, link);
            gem_links.push(gemlink);

            return format!("[{}]", link);
        })
        .to_string();

    // TODO: create memex syntax for linking to external gemini capsules

    output = MD_LINK_REGEX
        .replace_all(&output, |caps: &Captures| {
            let title = &caps[1];
            let link = &caps[2];

            let gemlink = format!("=> {} {}", link, title);
            http_links.push(gemlink);

            return format!("[{}]", title);
        })
        .to_string();

    TAG_SRC_REGEX.captures_iter(&output)
        .for_each(|x| {
            let src = &x[1];
            if src.starts_with("resources/") {
                http_links.push(format!("=> https://{}/{}", DOMAIN, src));
            } else {
                http_links.push(format!("=> {}", src));
            }
        });

    output = HTML_TAG_REGEX.replace_all(&output, "").to_string();

    output.push_str("\nGemini Links:\n");
    let links_str = String::from(gem_links.join("\n"));
    output.push_str(&links_str);

    output.push_str("\n\nWeb Links:\n");
    let links_str = String::from(http_links.join("\n"));
    output.push_str(&links_str);

    return output;
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

fn build(
    content_path: &str,
    template_path: &str,
    destination_path: &str,
    resources_path: &str,
) -> Result<()> {
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
                .for_each(|x| x.content = format_directory_page(&x.content, &directory));

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
                let res = fd.write_all(html.as_bytes());
                if res.is_err() {
                    return Err(res.err().unwrap())
                }


                // create gemtext files
                let gemtext = contents
                    .replace(make_template("directory").as_str(), formatted_directory.as_str())
                    .replace(make_template("recent").as_str(), recents.as_str());
                let gemtext = render_gemtext(&gemtext);

                let fname = format!("{}/{}.gmi", destination_path, entry.id);
                hey(format!("{} => {}", entry.id.yellow(), fname.green()));
                let mut fd = File::create(fname)?;
                let res = fd.write_all(gemtext.as_bytes());
                if res.is_err() {
                    return Err(res.err().unwrap())
                }
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
    channel
        .validate()
        .expect("should be able to validate rss.xml file...");

    return Ok(channel);
}

fn add_rss_post(rss_path: PathBuf) -> Result<()> {
    let mut channel = parse_rss(&rss_path)
        .expect("unable to parse rss");

    hey("Title?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    title.pop();

    hey("Link?");
    let mut link = String::new();
    stdin().read_line(&mut link).unwrap();
    link.pop();

    hey("Description?");
    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();
    description.pop();

    let guid = GuidBuilder::default()
        .build()
        .expect("unable to build new guid");


    let rfc2822 = Utc::now().to_rfc2822();

    let item = ItemBuilder::default()
        .title(title)
        .link(link)
        .description(description)
        .guid(guid)
        .pub_date(rfc2822.clone())
        .build()
        .expect("could not build item");

    channel.set_last_build_date(rfc2822.clone());

    let mut cloned = channel.clone();
    let mut items = channel.into_items();
    items.push(item);
    cloned.set_items(items);

    cloned.validate()
        .expect("could not validate");

    let writer = File::create(rss_path)
        .expect("could not open rss path for writing");

    cloned.pretty_write_to(writer, b' ', 2)
        .expect("could not write new post to rss file.");

    return Ok(())
}

fn watch(
    content_path: &str,
    template_path: &str,
    destination_path: &str,
    resources_path: &str,
) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(content_path, RecursiveMode::Recursive)?;
    watcher.watch(resources_path, RecursiveMode::Recursive)?;
    watcher.watch(template_path, RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.

    let watching = "watching for changes!".bright_magenta().to_string();
    hey(&watching);
    loop {
        match rx.recv() {
            Ok(event) => {
                hey("updating...");
                println!("{:#?}", event);
                build(
                    content_path,
                    template_path,
                    destination_path,
                    resources_path,
                )?;
                hey(&watching);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn rename(content_path: &str, old: &str, new: &str) -> Result<()> {
    let entries = collect_entries(content_path).expect("couldn't collect entries.");
    hey(format!("replacing {} with {}", old, new));

    let as_string = format!(r"\[\[{}\]\]", old);
    let regex = Regex::new(as_string.as_str()).expect("invalid replacement regex");

    for entry in entries {
        if regex.is_match(&entry.content) {
            let replaced = regex.replace_all(&entry.content, format!("[[{}]]", new));
            let path = Path::new(content_path).join(entry.path);
            hey(format!("writing to {:#?}", path.as_os_str()));
            let mut fd = File::create(path).expect("couldn't create new file");
            fd.write_all(replaced.as_bytes())
                .expect("couldn't write new file");
        }
    }

    return Ok(());
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
                .long("content")
                .help("sets the content folder"),
        )
        .arg(
            Arg::with_name("template")
                .short("t")
                .long("template")
                .help("sets the template"),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("destination")
                .help("sets the destination directory"),
        )
        .arg(
            Arg::with_name("resources")
                .short("r")
                .long("resources")
                .help("sets the resources directory"),
        )
        .subcommand(App::new("build").about("builds the memex"))
        .subcommand(App::new("rss").about("adds a new rss item"))
        .subcommand(
            App::new("watch")
                .about("watches for file system changes and builds the memex on each change"),
        )
        .subcommand(
            App::new("rename")
                .about("updates internal page id across entries")
                .arg(
                    Arg::with_name("old")
                        .required(true)
                        .long("old")
                        .short("o")
                        .help("old name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("new")
                        .required(true)
                        .long("new")
                        .short("n")
                        .help("new name")
                        .takes_value(true),
                ),
        )
        .get_matches();

    setup().expect("couldn't setup.");

    let content_path = matches.value_of("content").unwrap_or("content/entries");
    let template_path = matches
        .value_of("template")
        .unwrap_or("templates/base.html");
    let destination_path = matches.value_of("destination").unwrap_or("dist");
    let resources_path = matches.value_of("resources").unwrap_or("resources");

    if matches.subcommand_matches("build").is_some() {
        let b = build(
            content_path,
            template_path,
            destination_path,
            resources_path,
        );

        if b.is_ok() {
            hey("✨ Done!".bright_cyan().to_string());
        }

        return b;
    }

    if matches.subcommand_matches("watch").is_some() {
        let res = watch(
            content_path,
            template_path,
            destination_path,
            resources_path,
        );
        if res.is_err() {
            let msg = format!("error watching: {}", res.unwrap_err().to_string());
            return Err(Error::new(ErrorKind::Other, msg));
        }
    }

    if let Some(matches) = matches.subcommand_matches("rename") {
        let old = matches.value_of("old").expect("no old value provided");
        let new = matches.value_of("new").expect("no new value provided");
        return rename(content_path, old, new);
    }

    if  matches.subcommand_matches("rss").is_some() {
        let rss_path = Path::new("rss.xml").to_path_buf();
        return add_rss_post(rss_path);
    }

    let msg = "invalid command";
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
