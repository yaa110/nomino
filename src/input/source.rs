use crate::errors::SortOrderError;
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{self, Read};
use std::iter::Iterator;

#[derive(PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

pub enum Source {
    Regex(Regex),
    Map(Option<Vec<String>>, Option<Vec<String>>),
    Sort(SortOrder),
}

pub struct SourceIterator(Vec<String>, Option<Vec<String>>);

impl Source {
    pub fn new_regex(pattern: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Regex(Regex::new(pattern)?))
    }

    pub fn new_map(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut contents = String::new();
        File::open(filename).and_then(|mut file| file.read_to_string(&mut contents))?;
        let mut data: Vec<(String, String)> = serde_json::from_str(contents.as_str())?;
        let mut keys = Vec::with_capacity(data.len());
        let mut values = Vec::with_capacity(data.len());
        while let Some((k, v)) = data.pop() {
            keys.push(k);
            values.push(v);
        }
        Ok(Self::Map(Some(keys), Some(values)))
    }

    pub fn new_sort(order: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Sort(match order.to_lowercase().as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => return Err(Box::new(SortOrderError::new(order))),
        }))
    }

    pub fn try_iter(&mut self) -> Result<SourceIterator, Box<dyn Error>> {
        SourceIterator::try_from(self)
    }
}

impl SourceIterator {
    pub fn try_from(source: &mut Source) -> Result<Self, Box<dyn Error>> {
        if let Source::Map(keys, _) = source {
            return Ok(Self(keys.take().unwrap(), None));
        }

        let mut files = Vec::new();
        let mut captures = None;

        if let Source::Sort(order) = source {
            files = read_dir(".")?
                .map(|res| res.map(|e| e.path().to_string_lossy().to_string()))
                .collect::<Result<Vec<_>, io::Error>>()?;
            files.sort_by(|a, b| {
                if order == &SortOrder::Asc {
                    a.cmp(b)
                } else {
                    b.cmp(a)
                }
            });
        } else if let Source::Regex(regex) = source {
            // TODO regex
        }

        Ok(Self(files, captures))
    }
}

impl Iterator for SourceIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
