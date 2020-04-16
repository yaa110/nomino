use crate::errors::SortOrderError;
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub enum SortOrder {
    Asc,
    Desc,
}

pub enum Source {
    Regex(Regex),
    Map(HashMap<String, String>),
    Sort(SortOrder),
}

impl Source {
    pub fn new_regex(pattern: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Regex(Regex::new(pattern)?))
    }

    pub fn new_map(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut contents = String::new();
        File::open(filename).and_then(|mut file| file.read_to_string(&mut contents))?;
        Ok(Self::Map(serde_json::from_str(contents.as_str())?))
    }

    pub fn new_sort(order: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Sort(match order.to_lowercase().as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => return Err(Box::new(SortOrderError::new(order))),
        }))
    }
}
