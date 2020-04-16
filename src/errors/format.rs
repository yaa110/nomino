use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum FormatError {
    InvalidEscapeCharacter(usize, char),
    UnclosedPlaceholder,
    UnopenedPlaceholder,
    InvalidIndex(String),
    InvalidPadding(String),
    EmptyFormatter,
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::InvalidEscapeCharacter(pos, ch) => write!(
                f,
                "[output-format] invalid escape character of '{}' at '{}'",
                ch, pos
            ),
            FormatError::UnclosedPlaceholder => write!(
                f,
                "[output-format] all opened placeholders must be closed by '}}'"
            ),
            FormatError::UnopenedPlaceholder => write!(
                f,
                "[output-format] an unopened placeholder could not be closed by '}}'"
            ),
            FormatError::InvalidIndex(index) => {
                write!(f, "[output-format] unable to parse index of '{}'", index)
            }
            FormatError::InvalidPadding(padding) => write!(
                f,
                "[output-format] unable to parse padding of '{}'",
                padding
            ),
            FormatError::EmptyFormatter => {
                write!(f, "[output-format] output formatter must be set")
            }
        }
    }
}

impl Error for FormatError {}
