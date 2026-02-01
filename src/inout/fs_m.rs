use std::ffi::CString;

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
    pub fd: i32,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
