
use std::{fmt, io};

use thiserror::Error;
use crossterm::ErrorKind;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("IO Error")]
	IoError(#[from] io::Error),
	
	#[error("Fmt Error")]
	FmtError(#[from] fmt::Error),
	
	#[error("Utf8 Error")]
	Utf8Error(#[from] std::string::FromUtf8Error),
	
	#[error("Parse Int Error")]
	ParseIntError(#[from] std::num::ParseIntError),
	
	#[error("Resizing Terminal Error: {0}")]
	ResizingTerminalError(String),
	
	#[error("Setting Terminal Title Error")]
	SettingTerminalTitleError,
	
	#[error("Int Convert Error")]
	IntConvertError(#[from] std::num::TryFromIntError),
	
	#[error("{0}")]
	OtherError(String),
}

impl From<ErrorKind> for Error {
	fn from(up: ErrorKind) -> Error {
		match up {
			ErrorKind::IoError(err) => Error::IoError(err),
			ErrorKind::FmtError(err) => Error::FmtError(err),
			ErrorKind::Utf8Error(err) => Error::Utf8Error(err),
			ErrorKind::ParseIntError(err) => Error::ParseIntError(err),
			ErrorKind::ResizingTerminalFailure(msg) => Error::ResizingTerminalError(msg),
			ErrorKind::SettingTerminalTitleFailure => Error::SettingTerminalTitleError,
			_ => Error::OtherError(format!("{:?}", up))
		}
	}
}

impl From<std::convert::Infallible> for Error {
	fn from(_up: std::convert::Infallible) -> Error {
		unreachable!();
	}
}

impl <T> From<std::sync::mpsc::SendError<T>> for Error {
	fn from(up: std::sync::mpsc::SendError<T>) -> Error {
		Error::OtherError(format!("{:?}", up))
	}
}

impl <T> From<std::sync::PoisonError<T>> for Error {
	fn from(up: std::sync::PoisonError<T>) -> Error {
		Error::OtherError(format!("{:?}", up))
	}
}

impl Drop for Error {
	fn drop(&mut self) {
		eprintln!("{:?}", self);
	}
}
