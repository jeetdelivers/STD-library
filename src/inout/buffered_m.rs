use crate::inout::{Error, Read, Result, Write};
use  std::cmp;

const DEFAULT_BUFF_SIZE: usize = 8192;
pub struct BufReader<R: Read> {
    inner: R,
    buf: [u8; DEFAULT_BUFF_SIZE],
    pos: usize,
    cap: usize,
}

impl<R: Read> BufReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: [0; DEFAULT_BUFF_SIZE],
            pos: 0,
            cap: 0,
        }
    }
}

impl<R: Read> Read for BufReader<R> {
    fn read(&mut self, dest: &mut [u8]) -> Result<usize> {
        if self.pos == self.cap {
            if dest.len() >= DEFAULT_BUFF_SIZE {
                return self.inner.read(dest);
            }

            self.cap = self.inner.read(&mut self.buf)?;
            self.pos = 0;

            if self.cap == 0 {
                return Ok(0);
            }
        }

        let remaining = self.cap - self.pos;

        let to_copy = cmp::min(remaining, dest.len());

        dest[..to_copy].copy_from_slice(&self.buf[self.pos..self.pos + to_copy]);
        
        self.pos += to_copy;

        Ok(to_copy)
    }
}

pub struct BufWriter<W: Write> {
    inner: Option<W>,
    buf: Vec<u8>,
    len: usize,
}

impl<W: Write> BufWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner: Some(inner),
            buf: Vec::with_capacity(8192),
            len: 8192,
        }
    }
}

impl<W: Write> BufWriter<W> {
    fn flush_buf(&mut self) -> Result<()> {
        let mut written = 0;
        let len = self.buf.len();

        let inner = match &mut self.inner {
            Some(w) => w,
            None => return Err(Error::last_os_error()),
        };

        while written < len {
            match inner.write(&self.buf[written..]) {
                Ok(0) => return Err(Error::last_os_error()),
                Ok(n) => written += n,
                Err(e) => return Err(e),
            }
        }
    
        self.buf.clear();
        Ok(())
    }
}

impl<W: Write> Write for BufWriter<W> {
    fn write(&mut self, data: &[u8]) -> Result<usize>{
        if self.buf.len() + data.len() > self.len {
            self.flush_buf()?;

            if data.len() >= self.len {
                return self.inner.as_mut().unwrap().write(data);
            }
        }

        self.buf.extend_from_slice(data);

        Ok(data.len())
    }

    fn flush(&mut self) -> Result<usize>{
        self.flush_buf()?;
        
        self.inner.as_mut().unwrap().flush()
    }
}