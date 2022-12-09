use std::{error::Error, fmt};

pub fn str_to_vec(s: impl Into<String>, split: &str) -> Vec<String> {
    let val = s.into();

    return val
        .split(split)
        .filter_map(|line| line.parse::<String>().ok())
        .collect();
}

#[derive(Debug)]
pub struct ParseInputError;

impl Error for ParseInputError {}

impl fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse input")
    }
}
