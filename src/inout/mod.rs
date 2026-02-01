// src/io/mod.rs

// Declare the sub-modules
pub mod error_m;
pub mod traits_m;
pub mod buffered_m;
pub mod stdio_m;
pub mod fs_m;
pub mod util_m;
pub mod cursor_m;

// Re-export specific items to create a clean API
// (Note: The compiler will warn that these files are empty right now, that is okay)
pub use self::error_m::{Result, Error, ErrorKind};
pub use self::traits_m::{Read, Write, Seek, SeekFrom, BufRead};
pub use self::buffered_m::{BufReader, BufWriter};
pub use self::stdio_m::{stdin, stdout, stderr};
pub use self::fs_m::{File, OpenOptions};
pub use self::util_m::{copy, sink, empty};
pub use self::cursor_m::Cursor;

// The Prelude
pub mod prelude {
    pub use super::{Read, Write, BufRead, Seek};
}