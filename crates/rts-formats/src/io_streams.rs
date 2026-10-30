//! Buffer and file Pipe/Straw endpoints ported from REDALERT/XPIPE.CPP and XSTRAW.CPP.
//!
//! Buffer endpoints transfer bytes to/from fixed memory slices. File endpoints
//! are generic over Rust Read/Write implementations; path-backed endpoints
//! lazily open their file on the first non-empty request, matching the source
//! FilePipe/FileStraw open-on-demand behavior.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::pipe::PipeSink;
use crate::straw::StrawSource;

/// Fixed-buffer terminal Pipe corresponding to BufferPipe.
#[derive(Debug)]
pub struct BufferPipe<'a> {
    buffer: &'a mut [u8],
    index: usize,
}

impl<'a> BufferPipe<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, index: 0 }
    }

    pub const fn position(&self) -> usize { self.index }
    pub fn remaining(&self) -> usize { self.buffer.len().saturating_sub(self.index) }
    pub fn written(&self) -> &[u8] { &self.buffer[..self.index] }
}

impl PipeSink for BufferPipe<'_> {
    fn put(&mut self, source: &[u8]) -> usize {
        let count = source.len().min(self.remaining());
        if count != 0 {
            self.buffer[self.index..self.index + count].copy_from_slice(&source[..count]);
            self.index += count;
        }
        count
    }
}

/// Fixed-buffer source Straw corresponding to BufferStraw.
#[derive(Debug, Clone)]
pub struct BufferStraw<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> BufferStraw<'a> {
    pub const fn new(buffer: &'a [u8]) -> Self { Self { buffer, index: 0 } }
    pub const fn position(&self) -> usize { self.index }
    pub fn remaining(&self) -> usize { self.buffer.len().saturating_sub(self.index) }
}

impl StrawSource for BufferStraw<'_> {
    fn get(&mut self, destination: &mut [u8]) -> usize {
        let count = destination.len().min(self.remaining());
        if count != 0 {
            destination[..count].copy_from_slice(&self.buffer[self.index..self.index + count]);
            self.index += count;
        }
        count
    }
}

/// Push endpoint over an already-open Write implementation.
#[derive(Debug)]
pub struct FilePipe<W> {
    writer: W,
}

impl<W> FilePipe<W> {
    pub const fn new(writer: W) -> Self { Self { writer } }
    pub fn writer(&self) -> &W { &self.writer }
    pub fn writer_mut(&mut self) -> &mut W { &mut self.writer }
    pub fn into_inner(self) -> W { self.writer }
}

impl<W: Write> PipeSink for FilePipe<W> {
    fn put(&mut self, source: &[u8]) -> usize {
        self.writer.write(source).unwrap_or(0)
    }

    fn flush(&mut self) -> usize {
        let _ = self.writer.flush();
        0
    }
}

/// Pull endpoint over an already-open Read implementation.
#[derive(Debug)]
pub struct FileStraw<R> {
    reader: R,
}

impl<R> FileStraw<R> {
    pub const fn new(reader: R) -> Self { Self { reader } }
    pub fn reader(&self) -> &R { &self.reader }
    pub fn reader_mut(&mut self) -> &mut R { &mut self.reader }
    pub fn into_inner(self) -> R { self.reader }
}

impl<R: Read> StrawSource for FileStraw<R> {
    fn get(&mut self, destination: &mut [u8]) -> usize {
        if destination.is_empty() { return 0; }
        self.reader.read(destination).unwrap_or(0)
    }
}

/// Owned path-based FilePipe that opens/truncates on the first write.
#[derive(Debug)]
pub struct PathFilePipe {
    path: PathBuf,
    file: Option<File>,
    opened_by_pipe: bool,
}

impl PathFilePipe {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), file: None, opened_by_pipe: false }
    }

    pub fn path(&self) -> &Path { &self.path }
    pub const fn is_open(&self) -> bool { self.file.is_some() }

    fn open_for_write(&mut self) -> io::Result<&mut File> {
        if self.file.is_none() {
            self.file = Some(OpenOptions::new().write(true).create(true).truncate(true).open(&self.path)?);
            self.opened_by_pipe = true;
        }
        Ok(self.file.as_mut().expect("file was opened"))
    }

    /// Source FilePipe::End: flush, then close only a file opened by this pipe.
    pub fn end(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.flush()?;
        }
        if self.opened_by_pipe {
            self.file.take();
            self.opened_by_pipe = false;
        }
        Ok(())
    }
}

impl PipeSink for PathFilePipe {
    fn put(&mut self, source: &[u8]) -> usize {
        if source.is_empty() { return 0; }
        match self.open_for_write().and_then(|file| file.write(source)) {
            Ok(count) => count,
            Err(_) => 0,
        }
    }

    fn flush(&mut self) -> usize {
        if let Some(file) = self.file.as_mut() {
            let _ = file.flush();
        }
        0
    }

    fn end(&mut self) -> usize {
        let _ = PathFilePipe::end(self);
        0
    }
}

impl Drop for PathFilePipe {
    fn drop(&mut self) {
        if self.opened_by_pipe {
            let _ = self.end();
        }
    }
}

/// Owned path-based FileStraw that opens on the first non-empty read.
#[derive(Debug)]
pub struct PathFileStraw {
    path: PathBuf,
    file: Option<File>,
    open_attempted: bool,
}

impl PathFileStraw {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), file: None, open_attempted: false }
    }

    pub fn path(&self) -> &Path { &self.path }
    pub const fn is_open(&self) -> bool { self.file.is_some() }

    fn open_for_read(&mut self) -> io::Result<&mut File> {
        if self.file.is_none() && !self.open_attempted {
            self.open_attempted = true;
            self.file = Some(File::open(&self.path)?);
        }
        self.file.as_mut().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file source is unavailable"))
    }
}

impl StrawSource for PathFileStraw {
    fn get(&mut self, destination: &mut [u8]) -> usize {
        if destination.is_empty() { return 0; }
        match self.open_for_read().and_then(|file| file.read(destination)) {
            Ok(count) => count,
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn buffer_pipe_stops_at_capacity() {
        let mut buffer = [0; 5];
        let mut pipe = BufferPipe::new(&mut buffer);
        assert_eq!(pipe.put(b"abc"), 3);
        assert_eq!(pipe.put(b"def"), 2);
        assert_eq!(pipe.put(b"x"), 0);
        assert_eq!(pipe.written(), b"abcde");
    }

    #[test]
    fn buffer_straw_reads_until_buffer_exhaustion() {
        let mut straw = BufferStraw::new(b"westwood");
        let mut first = [0; 3];
        assert_eq!(straw.get(&mut first), 3);
        assert_eq!(&first, b"wes");
        let mut rest = [0; 8];
        assert_eq!(straw.get(&mut rest), 5);
        assert_eq!(&rest[..5], b"twood");
        assert_eq!(straw.get(&mut rest), 0);
    }

    #[test]
    fn file_endpoints_use_read_and_write_semantics() {
        let mut pipe = FilePipe::new(Vec::new());
        assert_eq!(pipe.put(b"payload"), 7);
        assert_eq!(pipe.flush(), 0);
        assert_eq!(pipe.into_inner(), b"payload");

        let mut straw = FileStraw::new(Cursor::new(b"source".to_vec()));
        let mut destination = [0; 16];
        assert_eq!(straw.get(&mut destination), 6);
        assert_eq!(&destination[..6], b"source");
    }
}
