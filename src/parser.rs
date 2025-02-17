//! Parsing flags from text.
//!
//! # Grammar
//! Format and parse a flags value as text using the following grammar:
//!
//! - _Flags:_ (_Whitespace_ _Flag_ _Whitespace_)`|`*
//! - _Flag:_ _Name_ | _Hex Number_
//! - _Name:_ The name of any defined flag
//! - _Hex Number_: `0x`([0-9a-fA-F])*
//! - _Whitespace_: (\s)*
//!
//! Flags values can be formatted as _Flags_ by iterating over them, formatting each yielded flags
//! value as a _Flag_. Any yielded flags value that sets exactly the bits of a defined flag with a
//! name should be formatted as a _Name_. Otherwise it must be formatted as a _Hex Number_.
//!
//! Text that is empty or whitespace is an empty flags value.
//!
//! ## Modes
//! Formatting and parsing supports three modes:
//!
//! - **Retain**: Formatting and parsing roundtrips exactly the bits of the source flags value.
//!   This is the default behavior.
//! - **Truncate**: Flags values are truncated before formatting, and truncated after parsing.
//! - **Strict**: A _Flag_ may only be formatted and parsed as a _Name_. _Hex numbers_ are not
//!   allowed. A consequence of this is that unknown bits and any bits that aren't in a contained
//!   named flag will be ignored. This is recommended for flags values serialized across API
//!   boundaries, like web services.
//!
//! Given the following flags type:
//!
//! ```rust
//! # use bitflag_attr::bitflag;
//! #[bitflag(u8)]
//! #[derive(Clone, Copy)]
//! enum Flags {
//!     A  = 0b0000_0001,
//!     B  = 0b0000_0010,
//!     AB = 0b0000_0011,
//!     C  = 0b0000_1100,
//! }
//! ```
//!
//! The following are examples of how flags values can be formatted using any mode:
//!
//! ```rust,ignore
//! 0b0000_0000 = ""
//! 0b0000_0001 = "A"
//! 0b0000_0010 = "B"
//! 0b0000_0011 = "A | B"
//! 0b0000_0011 = "AB"
//! 0b0000_1111 = "A | B | C"
//! ```
//!
//! Truncate mode will unset any unknown bits:
//!
//! ```rust,ignore
//! 0b1000_0000 = ""
//! 0b1111_1111 = "A | B | C"
//! 0b0000_1000 = "0x8"
//! ```
//!
//! Retain mode will include any unknown bits as a final _Flag_:
//!
//! ```rust,ignore
//! 0b1000_0000 = "0x80"
//! 0b1111_1111 = "A | B | C | 0xf0"
//! 0b0000_1000 = "0x8"
//! ```
//!
//! Strict mode will unset any unknown bits, as well as bits not contained in any defined named flags:
//!
//! ```rust,ignore
//! 0b1000_0000 = ""
//! 0b1111_1111 = "A | B | C"
//! 0b0000_1000 = ""
//! ```
//!
//! # Example
//!
//! As an example, this is how `Flags::A | Flags::B | 0x0c` can be represented as text:
//!
//! ```text
//! A | B | 0x0c
//! ```
//!
//! Alternatively, it could be represented without whitespace:
//!
//! ```text
//! A|B|0x0C
//! ```
//!
//! Note that identifiers are *case-sensitive*, so the following is **not equivalent**:
//!
//! ```text
//! a|b|0x0C
//! ```
#![allow(clippy::let_unit_value)]

use core::fmt::{self, Write};

use crate::{BitsPrimitive, Flags};

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

#[cfg(not(feature = "alloc"))]
use fmt::Display as ToString;

/// Write a flags value as text.
///
/// Any bits that aren't part of a contained flag will be formatted as a hex number.
pub fn to_writer<B: Flags>(flags: &B, mut writer: impl Write) -> Result<(), fmt::Error> {
    // A formatter for bitflags that produces text output like:
    //
    // A | B | 0xf6
    //
    // The names of set flags are written in a bar-separated-format,
    // followed by a hex number of any remaining bits that are set
    // but don't correspond to any flags.

    // Iterate over known flag values
    let mut first = true;
    let mut iter = flags.iter_names();
    for (name, _) in &mut iter {
        if !first {
            writer.write_str(" | ")?;
        }

        first = false;
        writer.write_str(name)?;
    }

    // Append any extra bits that correspond to flags to the end of the format
    let remaining = iter.remaining().bits();
    if remaining != B::Bits::EMPTY {
        if !first {
            writer.write_str(" | ")?;
        }

        write!(writer, "{remaining:#X}")?;
    }

    fmt::Result::Ok(())
}

/// Parse a flags value from text.
///
/// This function will fail on any names that don't correspond to defined flags.
/// Unknown bits will be retained.
pub fn from_text<B: Flags>(input: &str) -> Result<B, ParseError>
where
    B::Bits: ParseHex,
{
    let mut parsed_flags = B::empty();

    // If the input is empty then return an empty set of flags
    if input.trim().is_empty() {
        return Ok(parsed_flags);
    }

    for flag in input.split('|') {
        let flag = flag.trim();

        // If the flag is empty then we've got missing input
        if flag.is_empty() {
            return Err(ParseError::empty_flag());
        }

        // If the flag starts with `0x` then it's a hex number
        // Parse it directly to the underlying bits type
        let parsed_flag = if let Some(flag) = flag.strip_prefix("0x") {
            let bits =
                <B::Bits>::parse_hex(flag).map_err(|_| ParseError::invalid_hex_flag(flag))?;

            B::from_bits_retain(bits)
        }
        // Otherwise the flag is a name
        // The generated flags type will determine whether
        // or not it's a valid identifier
        else {
            B::from_name(flag).ok_or_else(|| ParseError::invalid_named_flag(flag))?
        };

        parsed_flags.set(parsed_flag);
    }

    Ok(parsed_flags)
}

/// Write a flags value as text, ignoring any unknown bits.
pub fn to_writer_truncate<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error> {
    to_writer(&B::from_bits_truncate(flags.bits()), writer)
}

/// Parse a flags value from text.
///
/// This function will fail on any names that don't correspond to defined flags.
/// Unknown bits will be ignored.
pub fn from_text_truncate<B: Flags>(input: &str) -> Result<B, ParseError>
where
    B::Bits: ParseHex,
{
    Ok(B::from_bits_truncate(from_text::<B>(input)?.bits()))
}

/// Write only the contained, defined, named flags in a flags value as text.
pub fn to_writer_strict<B: Flags>(flags: &B, mut writer: impl Write) -> Result<(), fmt::Error> {
    // This is a simplified version of `to_writer` that ignores
    // any bits not corresponding to a named flag

    let mut first = true;
    let mut iter = flags.iter_names();
    for (name, _) in &mut iter {
        if !first {
            writer.write_str(" | ")?;
        }

        first = false;
        writer.write_str(name)?;
    }

    fmt::Result::Ok(())
}

/// Parse a flags value from text.
///
/// This function will fail on any names that don't correspond to defined flags.
/// This function will fail to parse hex values.
pub fn from_text_strict<B: Flags>(input: &str) -> Result<B, ParseError> {
    // This is a simplified version of `from_str` that ignores
    // any bits not corresponding to a named flag

    let mut parsed_flags = B::empty();

    // If the input is empty then return an empty set of flags
    if input.trim().is_empty() {
        return Ok(parsed_flags);
    }

    for flag in input.split('|') {
        let flag = flag.trim();

        // If the flag is empty then we've got missing input
        if flag.is_empty() {
            return Err(ParseError::empty_flag());
        }

        // If the flag starts with `0x` then it's a hex number
        // These aren't supported in the strict parser
        if flag.starts_with("0x") {
            return Err(ParseError::invalid_hex_flag("unsupported hex flag value"));
        }

        let parsed_flag = B::from_name(flag).ok_or_else(|| ParseError::invalid_named_flag(flag))?;

        parsed_flags.set(parsed_flag);
    }

    Ok(parsed_flags)
}

/// Parse a value from a hex string.
pub trait ParseHex {
    /// Parse the value from hex.
    fn parse_hex(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}

/// An error encountered while parsing flags from text.
#[derive(Debug)]
pub struct ParseError(ParseErrorKind);

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
enum ParseErrorKind {
    EmptyFlag,
    InvalidNamedFlag {
        #[cfg(not(feature = "alloc"))]
        got: (),
        #[cfg(feature = "alloc")]
        got: String,
    },
    InvalidHexFlag {
        #[cfg(not(feature = "alloc"))]
        got: (),
        #[cfg(feature = "alloc")]
        got: String,
    },
}

impl ParseError {
    /// An invalid hex flag was encountered.
    pub fn invalid_hex_flag<T>(flag: T) -> Self
    where
        T: fmt::Display + ToString,
    {
        let _flag = flag;

        let got = {
            #[cfg(feature = "alloc")]
            {
                _flag.to_string()
            }
        };

        ParseError(ParseErrorKind::InvalidHexFlag { got })
    }

    /// A named flag that doesn't correspond to any on the flags type was encountered.
    pub fn invalid_named_flag<T>(flag: T) -> Self
    where
        T: fmt::Display + ToString,
    {
        let _flag = flag;

        let got = {
            #[cfg(feature = "alloc")]
            {
                _flag.to_string()
            }
        };

        ParseError(ParseErrorKind::InvalidNamedFlag { got })
    }

    /// A hex or named flag wasn't found between separators.
    pub const fn empty_flag() -> Self {
        ParseError(ParseErrorKind::EmptyFlag)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ParseErrorKind::InvalidNamedFlag { got } => {
                let _got = got;

                write!(f, "unrecognized named flag")?;

                #[cfg(feature = "alloc")]
                {
                    write!(f, " `{}`", _got)?;
                }
            }
            ParseErrorKind::InvalidHexFlag { got } => {
                let _got = got;

                write!(f, "invalid hex flag")?;

                #[cfg(feature = "alloc")]
                {
                    write!(f, " `{}`", _got)?;
                }
            }
            ParseErrorKind::EmptyFlag => {
                write!(f, "encountered empty flag")?;
            }
        }

        Ok(())
    }
}

impl core::error::Error for ParseError {}
