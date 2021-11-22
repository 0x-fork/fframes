use nom::error::{ContextError, Error, ErrorKind, ParseError};

#[derive(Debug)]
pub struct WebVttError {
    pub message: String,
}

impl ParseError<&str> for WebVttError {
    // on one line, we show the error code and the input that caused it
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        let message = format!("Looking for: {:?}, found: {:?}", kind, input);
        println!("{}", message);
        WebVttError { message }
    }

    // if combining multiple errors, we show them one after the other
    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        let message = format!(
            "{} Looking for: {:?}, found: {:?}",
            other.message, kind, input
        );
        println!("{}", message);
        WebVttError { message }
    }

    fn from_char(input: &str, c: char) -> Self {
        let message = format!("Looking for: {:?}, found: {:?}", c, input);
        println!("{}", message);
        WebVttError { message }
    }

    fn or(self, other: Self) -> Self {
        let message = format!(
            "Failure. This may happen because of {} or {}\n",
            self.message, other.message
        );
        println!("{}", message);
        WebVttError { message }
    }
}

impl ContextError<&str> for WebVttError {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        let message = format!("{}\"{}\":\t{:?}\n", other.message, ctx, input);
        println!("{}", message);
        WebVttError { message }
    }
}

impl From<nom::Err<Error<&str>>> for WebVttError {
    fn from(error: nom::Err<Error<&str>>) -> Self {
        match error {
            nom::Err::Error(Error { input, code }) => WebVttError::from_error_kind(input, code),
            nom::Err::Failure(Error { input, code }) => WebVttError::from_error_kind(input, code),
            nom::Err::Incomplete(_) => WebVttError {
                message: "Incomplete data, giving up parsing.".to_owned(),
            },
        }
    }
}
