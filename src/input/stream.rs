use crate::errors::{FormatError, SourceError};
use crate::input::{Formatter, SortOrder, Source};
use async_std::fs;
use async_std::fs::ReadDir;
use async_std::prelude::*;
use async_std::stream::Stream;
use regex::Regex;
use std::error::Error;
use std::iter::IntoIterator;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

pub enum InputStream {
    VectorStream(IntoIter<(String, String)>),
    DirectoryStream(Formatter, Regex, bool, ReadDir),
}

impl InputStream {
    pub async fn try_from(
        source: Source,
        formatter: Option<Formatter>,
        preserve_extension: bool,
    ) -> Result<Self, Box<dyn Error>> {
        if let Source::Map(map) = source {
            return Ok(Self::VectorStream(map.into_iter()));
        }

        let formatter = formatter.ok_or(FormatError::EmptyFormatter)?;

        let mut entries = fs::read_dir(".").await?;

        if let Source::Sort(order) = source {
            let mut map = Vec::new();
            let mut inputs = Vec::new();
            while let Some(entry) = entries.next().await {
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
            return Ok(Self::VectorStream(map.into_iter()));
        }

        if let Source::Regex(re) = source {
            return Ok(Self::DirectoryStream(
                formatter,
                re,
                preserve_extension,
                entries,
            ));
        }

        Err(Box::new(SourceError::new(String::from("unknown source"))))
    }
}

impl Stream for InputStream {
    type Item = (String, String);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.as_mut().get_mut() {
            Self::VectorStream(ref mut iter) => Poll::Ready(iter.next()),
            Self::DirectoryStream(ref formatter, ref re, preserve_extension, iter) => {
                match Pin::new(iter).poll_next(cx) {
                    Poll::Ready(Some(Ok(entry))) => {
                        let input = entry.file_name().to_string_lossy().to_string();
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
                            return Poll::Ready(Some((input, output)));
                        }
                        self.poll_next(cx)
                    }
                    Poll::Ready(Some(Err(_))) => self.poll_next(cx),
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}
