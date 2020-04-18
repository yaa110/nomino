use crate::errors::{FormatError, SourceError};
use crate::input::{Formatter, SortOrder, Source};
use regex::Regex;
use std::error::Error;
use std::fs::{self, ReadDir};
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::path::Path;
use std::vec::IntoIter;

pub enum InputIterator {
    VectorIterator(IntoIter<(String, String)>),
    DirectoryIterator(Formatter, Regex, bool, ReadDir),
}

impl InputIterator {
    pub fn try_from(
        source: Source,
        formatter: Option<Formatter>,
        preserve_extension: bool,
    ) -> Result<Self, Box<dyn Error>> {
        if let Source::Map(map) = source {
            return Ok(Self::VectorIterator(map.into_iter()));
        }

        let formatter = formatter.ok_or(FormatError::EmptyFormatter)?;

        let mut entries = fs::read_dir(".")?;

        if let Source::Sort(order) = source {
            let mut map = Vec::new();
            let mut inputs = Vec::new();
            while let Some(entry) = entries.next() {
                if let Ok(entry) = entry {
                    inputs.push(entry.file_name().to_string_lossy().to_string());
                }
            }
            inputs.sort_by(|a, b| {
                if order == SortOrder::Asc {
                    a.to_lowercase().as_str().cmp(b.to_lowercase().as_str())
                } else {
                    b.to_lowercase().as_str().cmp(a.to_lowercase().as_str())
                }
            });
            for (i, input) in inputs.into_iter().enumerate() {
                let index = (i + 1).to_string();
                let mut output = formatter.format(vec![input.as_str(), index.as_str()].as_slice());
                if preserve_extension {
                    if let Some(extension) = Path::new(input.as_str()).extension() {
                        output.push('.');
                        output.push_str(extension.to_str().unwrap_or_default());
                    }
                }
                map.push((input, output));
            }
            return Ok(Self::VectorIterator(map.into_iter()));
        }

        if let Source::Regex(re) = source {
            return Ok(Self::DirectoryIterator(
                formatter,
                re,
                preserve_extension,
                entries,
            ));
        }

        Err(Box::new(SourceError::new(String::from("unknown source"))))
    }
}

impl Iterator for InputIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::VectorIterator(ref mut iter) => iter.next(),
            Self::DirectoryIterator(ref formatter, ref re, preserve_extension, ref mut iter) => {
                for entry in iter {
                    let input = if let Ok(entry) = entry {
                        entry.file_name().to_string_lossy().to_string()
                    } else {
                        continue;
                    };
                    if let Some(cap) = re.captures(input.as_str()) {
                        let vars: Vec<&str> = cap
                            .iter()
                            .map(|c| c.map(|c| c.as_str()).unwrap_or_default())
                            .collect();
                        let mut output = formatter.format(vars.as_slice());
                        if *preserve_extension {
                            if let Some(extension) = Path::new(input.as_str()).extension() {
                                output.push('.');
                                output.push_str(extension.to_str().unwrap_or_default());
                            }
                        }
                        return Some((input, output));
                    }
                }
                None
            }
        }
    }
}
