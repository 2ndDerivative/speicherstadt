use std::{fmt::Display, str::FromStr};

use unicode_xid::UnicodeXID;

/// Some of these are probably not needed but I guess it's not necessary to take a chance
const WINDOWS_FORBIDDEN_FILE_NAMES: &[&str; 30] = &[
    "CON", "PRN", "AUX", "NUL", "COM0", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "COM¹",
    "COM²", "COM³", "LPT0", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9", "LPT¹", "LPT²",
    "LPT³",
];

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PackageName<T: AsRef<str> = String>(T);
impl<T> PackageName<T>
where
    T: AsRef<str>,
{
    fn new(inner: T) -> Result<Self, InvalidPackageName> {
        let mut chars = inner.as_ref().chars();
        match chars.next() {
            None => return Err(InvalidPackageName::Empty),
            Some(ch) if ch.is_ascii_digit() => return Err(InvalidPackageName::StartsWithDigit),
            Some(ch) if UnicodeXID::is_xid_start(ch) => {}
            Some('_') => {}
            _ => return Err(InvalidPackageName::InvalidStartCharacter),
        }
        for char in chars {
            match char {
                '-' | '_' => {}
                ch if UnicodeXID::is_xid_continue(ch) => {}
                _ => return Err(InvalidPackageName::InvalidContinueCharacter),
            }
        }
        if WINDOWS_FORBIDDEN_FILE_NAMES.contains(&inner.as_ref().to_ascii_uppercase().as_str()) {
            return Err(InvalidPackageName::ForbiddenWindowsFileName);
        }
        Ok(Self(inner))
    }
}
impl FromStr for PackageName {
    type Err = InvalidPackageName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PackageName::new(s.to_string())
    }
}
impl<T> Display for PackageName<T>
where
    T: Display + AsRef<str>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
#[derive(Debug)]
pub enum InvalidPackageName {
    Empty,
    StartsWithDigit,
    InvalidStartCharacter,
    InvalidContinueCharacter,
    ForbiddenWindowsFileName,
}
impl std::error::Error for InvalidPackageName {}
impl Display for InvalidPackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("package name is empty"),
            Self::StartsWithDigit => f.write_str("package name should not start with a digit"),
            Self::InvalidStartCharacter => f.write_str("package name doesn't start with Unicode XID start or '_'"),
            Self::InvalidContinueCharacter => f.write_str("package name body must be Unicode XID continue, '-' or '_'"),
            Self::ForbiddenWindowsFileName => f.write_str("package name must not be Windows-forbidden file name"),
        }
    }
}
