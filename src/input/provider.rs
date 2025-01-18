use regex::Captures;

#[derive(Debug, PartialEq)]
pub enum Capture {
    Index(usize),
    Name(String),
}

pub trait Provider {
    fn provide(&self, cap: &Capture) -> Option<&str>;
}

impl Provider for Captures<'_> {
    fn provide(&self, cap: &Capture) -> Option<&str> {
        match cap {
            Capture::Index(index) => self.get(*index),
            Capture::Name(name) => self.name(name.as_str()),
        }
        .map(|m| m.as_str())
    }
}

impl Provider for Vec<&'_ str> {
    fn provide(&self, cap: &Capture) -> Option<&str> {
        match cap {
            Capture::Index(index) => self.get(*index).copied(),
            _ => None,
        }
    }
}

impl From<&str> for Capture {
    fn from(value: &str) -> Self {
        if let Ok(index) = value.parse() {
            Capture::Index(index)
        } else {
            Capture::Name(value.into())
        }
    }
}
