use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use chrono::{Local, NaiveDate};

use crate::common::{load_file, nope, EpiStatus};
use crate::regexes;
use crate::SEEDLING;

#[derive(Debug, Clone)]
struct RawEntry {
    file_stem: OsString,
    content: String,
    path_buf: PathBuf,
}

/// Raw entry just holds a file step, content, and path.
impl RawEntry {
    fn from_path(path_buf: &PathBuf) -> Option<Self> {
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
        return Some(Self {
            file_stem: OsString::from(stem.unwrap()),
            content: content.unwrap(),
            path_buf: path_buf.clone(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub content: String,
    pub epistemic_status: EpiStatus,
    pub modification_date: String,
    pub links: Vec<String>,
    pub references: Option<Vec<String>>,
    pub outline_md: String,
    pub path: PathBuf,
}
impl Entry {
    fn get_links(content: &str) -> Vec<String> {
        regexes::INTERNAL_LINK_REGEX
            .captures_iter(content)
            .filter_map(|c| c.get(3))
            .map(|x| String::from(x.as_str()))
            .collect()
    }

    fn get_modification_string(path: &PathBuf) -> String {
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

    fn make_header_link(header: &str) -> String {
        let temp = regexes::HEADER_REGEX
            .replace_all(header, "$heading")
            // these should just disappear
            .replace("'", "")
            .replace("\"", "");

        // everthing else not a word becomes a dash
        let clean = regexes::NON_WORD_REGEX.replace_all(&temp.trim(), "-");
        return String::from(clean);
    }

    fn get_outline_md(content: &str) -> String {
        let mut result = String::from("");
        if regexes::HEADER_REGEX.is_match(content) {
            let headers = content
                .split("\n")
                .filter(|x| x.starts_with("#"))
                .collect::<Vec<&str>>();

            for line in headers {
                // calculate indent before cleaning
                let indent = "  ".repeat(line.matches("#").count());
                result.push_str(&indent);

                // replace anything that isn't a word char
                let header_link = Entry::make_header_link(line);
                let heading = regexes::HEADER_REGEX
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

    fn get_epistemic_status(content: &str) -> EpiStatus {
        let epistemic_status = match regexes::EPISTEMIC_REGEX.captures(content) {
            Some(cap) => cap
                .name("status")
                .expect("failed to extract epistemic status")
                .as_str(),
            _ => SEEDLING,
        };

        return EpiStatus::from_str(epistemic_status).expect(
            format!(
                "{}: {:?}",
                "unable to parse epistemic status", epistemic_status
            )
            .as_str(),
        );
    }

    /// The main entrypoint for this struct
    pub fn from_path(path_buf: &PathBuf) -> Option<Self> {
        match RawEntry::from_path(path_buf) {
            Some(raw) => {
                let content_str = raw.content.as_str();
                return Some(Entry {
                    id: String::from(raw.file_stem.to_str().unwrap()),
                    content: raw.content.clone(),
                    epistemic_status: Entry::get_epistemic_status(content_str),
                    links: Entry::get_links(content_str),
                    modification_date: Entry::get_modification_string(&raw.path_buf),
                    outline_md: Entry::get_outline_md(content_str),
                    references: None,
                    path: raw.path_buf.clone(),
                });
            }
            _ => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use regex::Regex;

    // import namespace above here
    use super::*;

    #[test]
    fn test_extract_content_from_path() {
        let path_buf = PathBuf::from("Cargo.toml");
        let res = RawEntry::from_path(&path_buf);
        assert!(res.is_some());
        assert!(res.unwrap().file_stem == "Cargo");
    }

    #[test]
    fn test_extract_references_from_content() {
        let res = Entry::get_links("testing [[one]] [[two]]");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "one");
        assert_eq!(res[1], "two");
    }

    #[test]
    fn test_extract_modification_date_from_path() {
        let path = Path::new("Cargo.toml").to_path_buf();
        let res = Entry::get_modification_string(&path).to_string();
        let regex = Regex::new(r"\d{4}.\d{2}.\d{2}").unwrap();
        let captures = regex.captures(res.as_str()).unwrap();
        assert_eq!(captures.len(), 1);
    }
}
