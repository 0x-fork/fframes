use nom::error::{ContextError, Error, ErrorKind, ParseError};

#[derive(Debug)]
pub struct WebVttError {
    /// What we are looking for
    pub looking_for: String,
    /// What we get
    pub input: String,
    /// Context-specific message
    pub message: Option<String>,
}

impl ParseError<&str> for WebVttError {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        WebVttError {
            message: None,
            looking_for: format!("{:?}", kind),
            input: input.to_owned(),
        }
    }

    fn append(input: &str, kind: ErrorKind, _other: Self) -> Self {
        WebVttError {
            message: None,
            looking_for: format!("{:?}", kind),
            input: input.to_owned(),
        }
    }

    fn from_char(input: &str, c: char) -> Self {
        WebVttError {
            message: None,
            looking_for: c.to_string(),
            input: input.to_owned(),
        }
    }

    fn or(self, other: Self) -> Self {
        let message = format!(
            "Failure. Looking for {} or {}\n",
            self.looking_for, other.looking_for
        );

        WebVttError {
            input: self.input.to_owned(),
            looking_for: self.looking_for,
            message: Some(message),
        }
    }
}

impl ContextError<&str> for WebVttError {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        WebVttError {
            message: Some(ctx.to_string()),
            input: input.to_owned(),
            looking_for: other.looking_for,
        }
    }
}

impl From<nom::Err<Error<&str>>> for WebVttError {
    fn from(error: nom::Err<Error<&str>>) -> Self {
        match error {
            nom::Err::Error(Error { input, code }) => WebVttError::from_error_kind(input, code),
            nom::Err::Failure(Error { input, code }) => WebVttError::from_error_kind(input, code),
            nom::Err::Incomplete(_) => WebVttError {
                input: "".to_owned(),
                looking_for: "".to_owned(),
                message: Some("Incomplete data, giving up parsing.".to_owned()),
            },
        }
    }
}
