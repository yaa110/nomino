use crate::errors::{FormatError, SourceError};
use crate::input::{Formatter, SortOrder, Source};
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::iter::{IntoIterator, Iterator};
use std::path::Path;
use std::vec::IntoIter;
use walkdir::IntoIter as WalkIter;
use walkdir::WalkDir;

pub enum InputIterator {
    VectorIterator(IntoIter<(String, String)>),
    DirectoryIterator {
        formatter: Formatter,
        re: Regex,
        preserve_extension: bool,
        iter: WalkIter,
    },
}

impl InputIterator {
    pub fn new(
        source: Source,
        formatter: Option<Formatter>,
        preserve_extension: bool,
    ) -> Result<Self> {
        if let Source::Map(map) = source {
            return Ok(Self::VectorIterator(map.into_iter()));
        }

        let formatter = formatter.ok_or(FormatError::EmptyFormatter)?;

        if let Source::Sort(order) = source {
            let mut map = Vec::new();
            let mut inputs = Vec::new();
            for entry in fs::read_dir(".")?.flatten() {
                inputs.push(entry.file_name().to_string_lossy().to_string());
            }
            inputs.sort_by(|a, b| {
                if order == SortOrder::Asc {
                    natord::compare(a, b)
                } else {
                    natord::compare(b, a)
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

        if let Source::Regex(re, depth, max_depth) = source {
            let max_depth = max_depth.unwrap_or(depth);
            return Ok(Self::DirectoryIterator {
                formatter,
                re,
                preserve_extension,
                iter: WalkDir::new(".")
                    .min_depth(if depth > max_depth { max_depth } else { depth })
                    .max_depth(max_depth)
                    .into_iter(),
            });
        }

        Err(SourceError::new(String::from("unknown source")).into())
    }
}

impl Iterator for InputIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::VectorIterator(iter) => iter.next(),
            Self::DirectoryIterator {
                formatter,
                re,
                preserve_extension,
                iter,
            } => {
                for entry in iter {
                    let Ok(entry) = entry else {
                        continue;
                    };
                    let path = entry.path();
                    let input = path.strip_prefix("./").unwrap_or(path).to_string_lossy();
                    let Some(cap) = re.captures(input.as_ref()) else {
                        continue;
                    };
                    let vars: Vec<&str> = cap
                        .iter()
                        .map(|c| c.map(|c| c.as_str()).unwrap_or_default())
                        .collect();
                    let mut output = formatter.format(vars.as_slice());
                    if *preserve_extension {
                        if let Some(extension) = Path::new(input.as_ref()).extension() {
                            output.push('.');
                            output.push_str(extension.to_str().unwrap_or_default());
                        }
                    }
                    return Some((input.to_string(), output));
                }
                None
            }
        }
    }
}
