use std::borrow::Borrow;
use std::io::{Read, Write};
use std::iter::Cycle;
use std::slice::Iter;

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for a in data.iter_mut() {
            *a ^= self.key.next().unwrap();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        Data: IntoIterator,
        Data::IntoIter: 'b,
        Data::Item: Borrow<u8>,
    {
        data.into_iter()
            .map(move |a| a.borrow() ^ *self.key.next().unwrap())
    }

    pub fn reader<R>(self, r: R) -> XorcismReader<'a, R>
    where
        R: Read,
    {
        XorcismReader {
            xorcism: self,
            reader: r,
        }
    }

    pub fn writer<W>(self, w: W) -> XorcismWriter<'a, W>
    where
        W: Write,
    {
        XorcismWriter {
            xorcism: self,
            writer: w,
        }
    }
}

pub struct XorcismReader<'a, R>
where
    R: Read,
{
    xorcism: Xorcism<'a>,
    reader: R,
}

impl<'a, R> Read for XorcismReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.reader.read(buf);
        self.xorcism.munge_in_place(buf);
        res
    }
}

pub struct XorcismWriter<'a, W>
where
    W: Write,
{
    xorcism: Xorcism<'a>,
    writer: W,
}

impl<'a, W> Write for XorcismWriter<'a, W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let x = self.xorcism.munge(buf).collect::<Vec<u8>>();
        self.writer.write(x.as_slice())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
