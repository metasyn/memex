pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Debug)]
pub enum Error {
    EndOfLine,
}

#[cfg(test)]
mod tests {
    // import namespace above here
    use super::*;
}
