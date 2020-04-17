use crate::errors::{FormatError, SourceError};
use crate::input::{Formatter, SortOrder, Source};
use async_std::fs;
use async_std::fs::ReadDir;
use async_std::prelude::*;
use async_std::stream::Stream;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use regex::Regex;
use std::error::Error;
use std::iter::IntoIterator;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

pub enum InputStream {
    VectorStream(IntoIter<(String, String)>),
    DirectoryStream(Formatter, Regex, ReadDir),
}

impl InputStream {
    pub async fn try_from(
        source: Source,
        formatter: Option<Formatter>,
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
                let output = formatter.format(vec![input.as_str(), index.as_str()].as_slice());
                map.push((input, output));
            }
            return Ok(Self::VectorStream(map.into_iter()));
        }

        if let Source::Regex(re) = source {
            return Ok(Self::DirectoryStream(formatter, re, entries));
        }

        Err(Box::new(SourceError::new(String::from("unknown source"))))
    }
}

impl Stream for InputStream {
    type Item = (String, String);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut() {
            Self::VectorStream(ref mut iter) => Poll::Ready(iter.next()),
            Self::DirectoryStream(ref formatter, ref re, iter) => {
                match Pin::new(iter).poll_next(cx) {
                    Poll::Ready(Some(Ok(entry))) => {
                        let input = entry.file_name().to_string_lossy().to_string();
                        if let Some(cap) = re.captures(input.as_str()) {
                            let vars: Vec<&str> = cap
                                .iter()
                                .par_bridge()
                                .map(|c| c.map(|c| c.as_str()).unwrap_or_default())
                                .collect();
                            let output = formatter.format(vars.as_slice());
                            return Poll::Ready(Some((input, output)));
                        }
                        Poll::Pending
                    }
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Pending | Poll::Ready(Some(Err(_))) => Poll::Pending,
                }
            }
        }
    }
}
