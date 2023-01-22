use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref INTERNAL_LINK_REGEX: Regex =
        Regex::new("(\\{(?P<title>.*)})?\\[\\[(?P<link>.+?)]]").unwrap();
    pub static ref HEADER_REGEX: Regex =
        Regex::new(r"^\s*(?P<level>#+)\s*(?P<heading>.*)").unwrap();
    pub static ref NON_WORD_REGEX: Regex = Regex::new(r"[^\w-]+").unwrap();
    pub static ref DITHERED_IMG_REGEX: Regex =
        Regex::new(r"(?P<img><img src=.*?resources/img/dithered_(?P<name>.+?)\..+?>)").unwrap();
    pub static ref MD_LINK_REGEX: Regex =
        Regex::new(r"\[(?P<title>.+?)\]\((?P<link>.+?)\)").unwrap();
    pub static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]*>").unwrap();
    pub static ref TAG_SRC_REGEX: Regex = Regex::new("src=\"(?P<src>.+?)\"").unwrap();
    pub static ref EPISTEMIC_REGEX: Regex = Regex::new(r"epistemic=(?P<status>\w+)").unwrap();
}
