use super::{provider::Capture, Provider};
use crate::errors::FormatError;

#[derive(Debug, PartialEq)]
enum Segment {
    PlaceHolder {
        padding: Option<usize>,
        capture: Capture,
    },
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Formatter(Vec<Segment>);

impl Formatter {
    pub fn new(format: &str) -> Result<Self, FormatError> {
        let mut segments = Vec::new();
        let mut should_escape = false;
        let mut is_parsing_capture = false;
        let mut is_parsing_padding = false;
        let mut current_segment = String::new();
        let mut current_capture = Capture::Index(0);
        let mut current_padding: Option<usize> = None;
        let mut incremental_index: usize = 1;
        for (i, ch) in format.chars().enumerate() {
            if !should_escape && ch == '\\' {
                should_escape = true;
                continue;
            }
            if should_escape && ch != '{' && ch != '}' && ch != '\\' {
                return Err(FormatError::InvalidEscapeCharacter(i, ch));
            }
            match ch {
                '{' if !should_escape && !is_parsing_capture && !is_parsing_padding => {
                    if !current_segment.is_empty() {
                        segments.push(Segment::String(current_segment));
                        current_segment = String::new();
                    }
                    is_parsing_capture = true;
                }
                '}' if !should_escape => {
                    if !is_parsing_capture && !is_parsing_padding {
                        return Err(FormatError::UnopenedPlaceholder);
                    }
                    if current_segment.is_empty() {
                        if is_parsing_capture {
                            current_capture = Capture::Index(incremental_index);
                            incremental_index += 1;
                        } else if is_parsing_padding {
                            current_padding = None;
                        }
                    } else if is_parsing_capture {
                        current_capture = current_segment.as_str().into();
                        current_padding = None;
                    } else if is_parsing_padding {
                        current_padding =
                            Some(current_segment.as_str().parse().map_err(|_| {
                                FormatError::InvalidPadding(current_segment.clone())
                            })?);
                    }
                    segments.push(Segment::PlaceHolder {
                        padding: current_padding,
                        capture: current_capture,
                    });
                    current_segment.clear();
                    current_padding = None;
                    current_capture = Capture::Index(0);
                    is_parsing_capture = false;
                    is_parsing_padding = false;
                }
                ':' if is_parsing_capture => {
                    is_parsing_capture = false;
                    is_parsing_padding = true;
                    if current_segment.is_empty() {
                        current_capture = Capture::Index(incremental_index);
                        incremental_index += 1;
                    } else {
                        current_capture = current_segment.as_str().into();
                        current_segment.clear();
                    }
                }
                _ => {
                    current_segment.push(ch);
                    should_escape = false;
                }
            }
        }
        if is_parsing_capture || is_parsing_padding {
            return Err(FormatError::UnclosedPlaceholder);
        }
        if !current_segment.is_empty() {
            segments.push(Segment::String(current_segment));
        }
        Ok(Self(segments))
    }

    pub fn format(&self, provider: impl Provider) -> String {
        let mut formatted = String::new();
        for segment in self.0.as_slice() {
            match segment {
                Segment::PlaceHolder { padding, capture } => {
                    let Some(var) = provider.provide(capture) else {
                        continue;
                    };
                    if let Some((padding, digits)) =
                        padding.zip(var.parse().map(|n: usize| n.to_string()).ok())
                    {
                        if digits.len() < padding {
                            let diff = padding - digits.len();
                            (0..diff).for_each(|_| formatted.push('0'));
                        }
                        formatted.push_str(digits.as_str());
                        continue;
                    }
                    formatted.push_str(var);
                }
                Segment::String(ref string) => formatted.push_str(string),
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
            ("{}", vec!["first", "second"], "second"),
            (r"{1}\\{0}", vec!["first", "second"], r"second\first"),
            (r"{1}\\\{{0}\}", vec!["first", "second"], r"second\{first}"),
            ("{}{}{3}", vec!["first", "second"], "second"),
            ("{1}", vec!["first", "second"], "second"),
            (
                "{1}:{1}.{1}",
                vec!["first", "second"],
                "second:second.second",
            ),
            ("{:3}", vec!["0", "1"], "001"),
            ("{:3}", vec!["0", "-1"], "-1"),
            ("{:3}", vec!["0", "a"], "a"),
            ("{:2}{:1}", vec!["0", "1", "2"], "012"),
            ("{1:3}", vec!["1", "2"], "002"),
            ("{}.{}", vec!["first", "second", "third"], "second.third"),
            ("{1}.{0}", vec!["first", "second"], "second.first"),
            ("{1}.{}", vec!["first", "second"], "second.second"),
            (
                "{2} - {} - {} - {}",
                vec!["first", "second", "third", "fourth"],
                "third - second - third - fourth",
            ),
            (
                "init {}{} end",
                vec!["first", "second", "third"],
                "init secondthird end",
            ),
            (
                r"init \{{}\} end",
                vec!["first", "second"],
                "init {second} end",
            ),
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
                vec!["0", "1", "2"],
                "init 00001{}02 end",
            ),
        ];

        while let Some((format, vars, expected)) = format_vars_expected.pop() {
            let output = Formatter::new(format)
                .expect(format!("unable to parse format '{}'", format).as_str());
            let actual = output.format(vars);
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
            ("{2:5a}", FormatError::InvalidPadding("5a".to_string())),
            ("init {2:5", FormatError::UnclosedPlaceholder),
            ("init {2:5 end", FormatError::UnclosedPlaceholder),
        ];

        while let Some((format, err)) = format_error.pop() {
            assert_eq!(Formatter::new(format), Err(err));
        }
    }
}
