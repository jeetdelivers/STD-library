use std::error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Other,
    UnexpectedEof,
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

#[derive(Debug)]
enum Repr {
    Os(i32),
    Simple(ErrorKind),
    Custom(Box<Custom>),
}

pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        let custom = Custom {
            kind,
            error: error.into(),
        };

        Error {
            repr: Repr::Custom(Box::new(custom)),
        }
    }

    pub fn from_raw_os_error(code: i32) -> Error {
        Error {
            repr: Repr::Os(code),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        match &self.repr {
            Repr::Os(code) => decode_error_kind(*code),
            Repr::Simple(kind) => *kind,
            Repr::Custom(c) => c.kind,
        }
    }
}

fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno {
        libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
        libc::ECONNRESET => ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
        libc::EPIPE => ErrorKind::BrokenPipe,
        libc::ENOTCONN => ErrorKind::NotConnected,
        libc::EWOULDBLOCK => ErrorKind::WouldBlock,
        libc::ENOENT => ErrorKind::NotFound,
        _ => ErrorKind::Other,
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Os(code) => write!(f, "os error {}", code),
            Repr::Simple(kind) => write!(f, "{:?}", kind),
            Repr::Custom(c) => c.error.fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn last_os_error() -> Error {
        let errno = unsafe { *libc::__errno_location() };
        Error::from_raw_os_error(errno)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.repr {
            Repr::Os(_) => None,
            Repr::Simple(_) => None,
            Repr::Custom(c) => Some(c.error.as_ref()),
        }
    }
}
