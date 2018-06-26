use failure::{Backtrace, Context, Fail};

/// An error that can occur in this crate.
///
/// Carries the error kind.
#[derive(Debug, Display)]
#[display(fmt = "{}", inner)]
pub struct Error {
    inner: Context<ErrorKind>,
}

/// The specific kind of error that can occur in this crate.
///
/// These variants do not have data, instead this should be carried by the context.
#[derive(Copy, Clone, Debug, Display, Eq, Fail, PartialEq)]
pub enum ErrorKind {
    /// The internal cursor over the data returned an IO error.
    #[display(fmt = "the internal cursor over the data returned an IO error")]
    IoError,

    /// The given data is not a complete Radiotap capture.
    #[display(fmt = "the given data is not a complete Radiotap capture")]
    IncompleteError,

    /// The given data is shorter than the amount specified in the Radiotap header.
    #[display(fmt = "the given data is shorter than the amount specified in the Radiotap header")]
    InvalidLength,

    /// The given data is not a valid Radiotap capture.
    #[display(fmt = "the given data is not a valid Radiotap capture")]
    InvalidFormat,

    /// Unsupported Radiotap header version.
    #[display(fmt = "unsupported Radiotap header version")]
    UnsupportedVersion,

    /// Unsupported Radiotap field.
    #[display(fmt = "unsupported Radiotap field")]
    UnsupportedField,

    /// Hints that destructuring should not be exhaustive.
    #[doc(hidden)]
    #[display(fmt = "this error should not be used")]
    __Nonexhaustive,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Error {
    fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}
