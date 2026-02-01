use std::ffi::CString;

use crate::inout::{self, Error};

pub struct OpenOptions {
    access_mode: i32,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self { access_mode: 0 }
    }

    pub fn read(&mut self, set: bool) -> &mut Self {
        if set {
            self.access_mode |= libc::O_RDONLY;
        } else {
            self.access_mode &= !libc::O_RDONLY;
        }
        self
    }

    pub fn write(&mut self, set: bool) -> &mut Self {
        if set {
            self.access_mode |= libc::O_WRONLY;
        } else {
            self.access_mode &= !libc::O_WRONLY;
        }
        self
    }

    pub fn open(&self, path: &str) -> Result<File, String> {
        let c_path = CString::new(path).unwrap();

        let fd = unsafe { libc::open(c_path.as_ptr(), self.access_mode) };

        if fd < 0 {
            Err("Failed to open".to_string())
        } else {
            Ok(File { fd })
        }
    }
}

pub struct File {
    fd: i32,
}

impl File {
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let bytes_read =
            unsafe { libc::read(
                self.fd, 
                buf.as_mut_ptr() as *mut libc::c_void, 
                buf.len()
            ) 
        };
        if bytes_read < 0 {
            return Err(inout::Error::last_os_error())
        } 

        Ok(bytes_read as usize)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
