use std::{fmt::Debug, ops::RangeInclusive};

use codepage_437::Cp437Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("libusb error: {0}")]
    USB(#[from] rusb::Error),
    #[error("no bulk endpoint could be found")]
    NoBulkEndpoint,
}

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("failed to write to printer: {0}")]
    USB(#[from] rusb::Error),
}

#[derive(Error, Debug)]
pub enum TextError<S: AsRef<str> + Debug> {
    #[error("{0}")]
    Printer(#[from] PrinterError),
    #[error("failed to convert text to Cp437: Invalid character '{}' at position {}", char::from_u32(.text.as_ref().as_bytes()[.err.representable_up_to] as u32).unwrap_or('?'), .err.representable_up_to)]
    Cp437 { text: S, err: Cp437Error },
}

#[derive(Error, Debug)]
pub enum BarcodeError<D: AsRef<[u8]> + Debug> {
    #[error("{0}")]
    Printer(#[from] PrinterError),
    #[error("invalid character '{}' at position {position}. Must be one of: {range:?}", data.as_ref()[*.position])]
    InvalidChar {
        range: RangeInclusive<u8>,
        data: D,
        position: usize,
    },
    #[error("invalid data size. Expected length: {expected:?}, provided length: {provided}")]
    InvalidSize {
        expected: RangeInclusive<usize>,
        provided: usize,
    },
}
