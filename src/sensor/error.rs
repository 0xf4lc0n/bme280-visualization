use std::fmt;
use std::sync::mpsc::SendError;

#[derive(Debug)]
pub struct Error {
	pub description: String,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl<T> From<SendError<T>> for Error {
	fn from(err: SendError<T>) -> Self {
		Error {
			description: err.to_string(),
		}
	}
}
