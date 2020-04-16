use crate::errors::FormatError;

#[derive(Debug, PartialEq)]
enum Segment {
    PlaceHolder {
        padding: Option<usize>,
        index: usize,
    },
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Output(Vec<Segment>);

impl Output {
    pub fn new(format: &str) -> Result<Self, FormatError> {
        let mut segments = Vec::new();
        let mut should_escape = false;
        let mut is_parsing_index = false;
        let mut is_parsing_padding = false;
        let mut current_segment = String::new();
        let mut current_index: usize = 0;
        let mut current_padding: Option<usize> = None;
        let mut incremental_index = 0;
        for (i, ch) in format.chars().enumerate() {
            if ch == '\\' {
                should_escape = true;
                continue;
            }
            if should_escape && ch != '{' && ch != '}' {
                return Err(FormatError::InvalidEscapeCharacter(i, ch));
            }
            match ch {
                '{' if !should_escape && !is_parsing_index && !is_parsing_padding => {
                    if !current_segment.is_empty() {
                        segments.push(Segment::String(current_segment));
                        current_segment = String::new();
                    }
                    is_parsing_index = true;
                }
                '}' if !should_escape => {
                    if !is_parsing_index && !is_parsing_padding {
                        return Err(FormatError::UnopenedPlaceholder);
                    }
                    if current_segment.is_empty() {
                        if is_parsing_index {
                            current_index = incremental_index;
                            incremental_index += 1;
                        } else if is_parsing_padding {
                            current_padding = None;
                        }
                    } else {
                        if is_parsing_index {
                            current_index = current_segment
                                .as_str()
                                .parse()
                                .map_err(|_| FormatError::InvalidIndex(current_segment.clone()))?;
                            current_padding = None;
                        } else if is_parsing_padding {
                            current_padding =
                                Some(current_segment.as_str().parse().map_err(|_| {
                                    FormatError::InvalidPadding(current_segment.clone())
                                })?);
                        }
                    }
                    segments.push(Segment::PlaceHolder {
                        padding: current_padding,
                        index: current_index,
                    });
                    current_segment.clear();
                    current_padding = None;
                    current_index = 0;
                    is_parsing_index = false;
                    is_parsing_padding = false;
                }
                ':' if is_parsing_index => {
                    is_parsing_index = false;
                    is_parsing_padding = true;
                    if current_segment.is_empty() {
                        current_index = incremental_index;
                        incremental_index += 1;
                    } else {
                        current_index = current_segment
                            .as_str()
                            .parse()
                            .map_err(|_| FormatError::InvalidIndex(current_segment.clone()))?;
                        current_segment.clear();
                    }
                }
                _ => {
                    current_segment.push(ch);
                    should_escape = false;
                }
            }
        }
        if is_parsing_index || is_parsing_padding {
            return Err(FormatError::UnclosedPlaceholder);
        }
        if !current_segment.is_empty() {
            segments.push(Segment::String(current_segment));
        }
        Ok(Self(segments))
    }

    pub fn format(&self, vars: &[&str]) -> String {
        let mut formatted = String::new();
        for segment in self.0.as_slice() {
            match segment {
                Segment::PlaceHolder { padding, index } => {
                    if let Some(var) = vars.get(*index) {
                        if let Some(padding) = *padding {
                            if let Ok(number) = var.parse::<usize>() {
                                let digits = number.to_string();
                                if digits.len() < padding {
                                    let diff = padding - digits.len();
                                    for i in 0..diff {
                                        formatted.push('0');
                                    }
                                }
                                formatted.push_str(digits.as_str());
                                continue;
                            }
                        }
                        formatted.push_str(var);
                    }
                }
                Segment::String(ref string) => formatted.push_str(string.as_str()),
            }
        }
        formatted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_formats() {
        let mut format_vars_expected = vec![
            ("{}", vec!["first"], "first"),
            ("{}{}{3}", vec!["first"], "first"),
            ("{1}", vec!["first", "second"], "second"),
            (
                "{1}:{1}.{1}",
                vec!["first", "second"],
                "second:second.second",
            ),
            ("{:2}", vec!["1"], "01"),
            ("{:2}{:1}", vec!["1", "2"], "012"),
            ("{1:3}", vec!["1", "2"], "002"),
            ("{}.{}", vec!["first", "second"], "first.second"),
            ("{1}.{0}", vec!["first", "second"], "second.first"),
            ("{1}.{}", vec!["first", "second"], "second.first"),
            (
                "{1} - {} - {} - {}",
                vec!["first", "second", "third"],
                "second - first - second - third",
            ),
            (
                "init {}{} end",
                vec!["first", "second"],
                "init firstsecond end",
            ),
            (r"init \{{}\} end", vec!["first"], "init {first} end"),
            (
                r"init \{{1:2}:{0:2}\} end",
                vec!["1", "2"],
                "init {02:01} end",
            ),
            (
                r"init \{{1:2}\{\}\{:\}{0:2}\} end",
                vec!["1", "2"],
                "init {02{}{:}01} end",
            ),
            (
                r"init {:5}\{\}{:2} end",
                vec!["1", "2"],
                "init 00001{}02 end",
            ),
        ];

        while let Some((format, vars, expected)) = format_vars_expected.pop() {
            let output =
                Output::new(format).expect(format!("unable to parse format '{}'", format).as_str());
            let actual = output.format(vars.as_slice());
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_invalid_formats() {
        let mut format_error = vec![
            ("}", FormatError::UnopenedPlaceholder),
            (r"\a", FormatError::InvalidEscapeCharacter(1, 'a')),
            ("2:5}", FormatError::UnopenedPlaceholder),
            (r"\{2:5}", FormatError::UnopenedPlaceholder),
            (r"{2:5\}", FormatError::UnclosedPlaceholder),
            ("{{2:5}}", FormatError::InvalidIndex("{2".to_string())),
            ("{a}", FormatError::InvalidIndex("a".to_string())),
            ("{2:5a}", FormatError::InvalidPadding("5a".to_string())),
            ("init {2:5", FormatError::UnclosedPlaceholder),
            ("init {2:5 end", FormatError::UnclosedPlaceholder),
        ];

        while let Some((format, err)) = format_error.pop() {
            assert_eq!(Output::new(format), Err(err));
        }
    }
}
