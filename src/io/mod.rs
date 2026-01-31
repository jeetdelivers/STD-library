// src/io/mod.rs

// Declare the sub-modules
pub mod error;
pub mod traits;
pub mod buffered;
pub mod stdio;
pub mod fs;
pub mod util;
pub mod cursor;

// Re-export specific items to create a clean API
// (Note: The compiler will warn that these files are empty right now, that is okay)
pub use self::error::{Result, Error, ErrorKind};
pub use self::traits::{Read, Write, Seek, SeekFrom, BufRead};
pub use self::buffered::{BufReader, BufWriter};
pub use self::stdio::{stdin, stdout, stderr};
pub use self::fs::{File, OpenOptions};
pub use self::util::{copy, sink, empty};
pub use self::cursor::Cursor;

// The Prelude
pub mod prelude {
    pub use super::{Read, Write, BufRead, Seek};
}