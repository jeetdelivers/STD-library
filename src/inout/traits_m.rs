use crate::inout::{self, File, Result};

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<usize>;
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
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

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written=
            unsafe { libc::write(
                self.fd, 
                buf.as_ptr() as *mut libc::c_void, 
                buf.len()
            ) 
        };
        if bytes_written < 0 {
            return Err(inout::Error::last_os_error())
        } 

        Ok(bytes_written as usize)
    }

    fn flush(&mut self) -> Result<usize> {
        Ok(unsafe { libc::fsync(self.fd)} as usize)
    }
}