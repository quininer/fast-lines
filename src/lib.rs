#![cfg_attr(feature = "nightly", feature(copy_within, read_initializer, try_reserve))]

use std::io::{ self, Read };
use memchr::memchr;

const LINE_FEED: u8 = b'\n';
const CARRIAGE_RETURN: u8 = b'\r';
const GROW_SIZE: usize = 512;


#[derive(Debug)]
pub struct ReadLine<R> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
    sol: usize,
    eol: usize,
    eof: bool
}

impl<R: Read> ReadLine<R> {
    #[inline]
    pub fn new(reader: R) -> ReadLine<R> {
        ReadLine {
            reader,
            buf: vec![0; GROW_SIZE],
            pos: 0,
            sol: 0,
            eol: 0,
            eof: false
        }
    }

    pub fn into_inner(self) -> R {
        self.reader
    }

    pub fn advance(&mut self) -> io::Result<()> {
        if self.eol == self.pos {
            self.pos = 0;
            self.eol = 0;
        }

        self.sol = self.eol;

        if self.eof {
            return Ok(());
        }

        if let Some(nl) = memchr(LINE_FEED, &self.buf[self.eol..self.pos]) {
            self.eol += nl + 1;

            Ok(())
        } else {
            if self.pos == self.buf.len() {
                let len = self.pos - self.eol;
                if self.sol < len {
                    #[cfg(feature = "nightly")] unsafe {
                        let len = self.buf.len();
                        self.buf.try_reserve(GROW_SIZE)
                            .map_err(|_| io::Error::new(io::ErrorKind::Other, "oom"))?;
                        self.buf.set_len(self.buf.capacity());
                        self.reader.initializer().initialize(&mut self.buf[len..]);
                    }

                    #[cfg(not(feature = "nightly"))]
                    self.buf.resize(self.buf.len() + GROW_SIZE, 0);
                } else {
                    #[cfg(feature = "nightly")]
                    self.buf.copy_within(self.eol..self.pos, 0);

                    #[cfg(not(feature = "nightly"))]
                    safemem::copy_over(&mut self.buf[..], self.eol, 0, len);

                    self.sol = 0;
                    self.eol = 0;
                    self.pos = len;
                }
            }

            match self.reader.read(&mut self.buf[self.pos..])? {
                0 => {
                    self.eol = self.pos;
                    self.eof = true;
                    Ok(())
                },
                n => {
                    self.pos += n;
                    self.advance()
                }
            }
        }
    }

    pub fn get(&self) -> Option<&[u8]> {
        // TODO eliminate bounds check

        let len = self.eol.checked_sub(self.sol).filter(|&n| n > 0)?;
        let eol = if len >= 1 && self.buf[self.eol - 1] == LINE_FEED {
            if len >= 2 && self.buf[self.eol - 2] == CARRIAGE_RETURN {
                self.eol - 2
            } else {
                self.eol - 1
            }
        } else {
            self.eol
        };

        Some(&self.buf[self.sol..eol])
    }

    pub fn read_line(&mut self) -> io::Result<Option<&[u8]>> {
        self.advance()?;
        Ok(self.get())
    }
}
