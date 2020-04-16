use crate::errors::SortOrderError;
use async_std::fs;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_std::task::{self, Context, Poll};
use regex::Regex;
use serde_json;
use std::error::Error;
use std::pin::Pin;

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
    pub async fn new_regex(pattern: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Regex(Regex::new(pattern)?))
    }

    pub async fn new_map(filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filename).await?;
        let mut data: Vec<(String, String)> =
            task::spawn(async move { serde_json::from_str(contents.as_str()) }).await?;
        Ok(task::spawn(async move {
            let mut keys = Vec::with_capacity(data.len());
            let mut values = Vec::with_capacity(data.len());
            while let Some((k, v)) = data.pop() {
                keys.push(k);
                values.push(v);
            }
            Self::Map(Some(keys), Some(values))
        })
        .await)
    }

    pub async fn new_sort(order: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Sort(match order.to_lowercase().as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => return Err(Box::new(SortOrderError::new(order))),
        }))
    }

    pub async fn try_iter(&mut self) -> Result<SourceIterator, Box<dyn Error>> {
        SourceIterator::try_from(self).await
    }
}

impl SourceIterator {
    pub async fn try_from(source: &mut Source) -> Result<Self, Box<dyn Error>> {
        if let Source::Map(keys, _) = source {
            return Ok(Self(keys.take().unwrap(), None));
        }

        let mut files = Vec::new();
        let mut captures = None;

        if let Source::Sort(order) = source {
            let mut entries = fs::read_dir(".").await?;
            while let Some(res) = entries.next().await {
                files.push(res?.path().to_string_lossy().to_string());
            }
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

impl Stream for SourceIterator {
    type Item = String;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.pop())
    }
}
