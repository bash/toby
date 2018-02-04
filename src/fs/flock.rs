use fs2::FileExt;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::fs::File;

pub struct FileLock {
    f: File,
}

impl FileLock {
    pub fn exclusive(f: File) -> io::Result<Self> {
        f.lock_exclusive()?;

        Ok(FileLock { f })
    }
}

impl Read for FileLock {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.f.read(buf)
    }
}

impl Seek for FileLock {
    fn seek(&mut self, to: SeekFrom) -> io::Result<u64> {
        self.f.seek(to)
    }
}

impl Write for FileLock {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.f.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.f.flush()
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = self.f.unlock();
    }
}