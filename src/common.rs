use colored::*;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;
use std::str::FromStr;

pub static DOMAIN: &str = "metasyn.pw";
pub const SEEDLING: &str = "seedling";
pub const SAPLING: &str = "sapling";
pub const DENDROID: &str = "dendroid";

pub fn hey<T>(msg: T)
where
    T: AsRef<str> + Display,
{
    println!("{} {}", "[INFO]".bright_cyan().bold(), msg)
}

pub fn nope<T>(msg: T)
where
    T: AsRef<str> + Display,
{
    println!("{} {}", "[ERROR]".bright_red().bold(), msg)
}

pub fn load_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path).expect("unable to open file path");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("unable to read file to string");
    return Ok(contents);
}

pub type EpiStatusLookup = HashMap<String, EpiStatus>;

#[derive(Debug, Clone)]
pub enum EpiStatus {
    Seedling,
    Sapling,
    Dendroid,
}

impl Display for EpiStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EpiStatus::Seedling => write!(f, "{}", SEEDLING),
            EpiStatus::Sapling => write!(f, "{}", SAPLING),
            EpiStatus::Dendroid => write!(f, "{}", DENDROID),
        }
    }
}

impl FromStr for EpiStatus {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            SEEDLING => Ok(EpiStatus::Seedling),
            SAPLING => Ok(EpiStatus::Sapling),
            DENDROID => Ok(EpiStatus::Dendroid),
            _ => Err(()),
        }
    }
}
