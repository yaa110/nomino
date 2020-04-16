use crate::errors::SortOrderError;
use async_std::{fs, task};
use regex::Regex;
use serde_json;
use std::error::Error;

#[derive(PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

pub enum Source {
    Regex(Regex),
    Map(Vec<(String, String)>),
    Sort(SortOrder),
}

impl Source {
    pub async fn new_regex(pattern: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Regex(Regex::new(pattern)?))
    }

    pub async fn new_map(filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filename).await?;
        Ok(Self::Map(
            task::spawn(async move { serde_json::from_str(contents.as_str()) }).await?,
        ))
    }

    pub async fn new_sort(order: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Sort(match order.to_lowercase().as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => return Err(Box::new(SortOrderError::new(order))),
        }))
    }
}
