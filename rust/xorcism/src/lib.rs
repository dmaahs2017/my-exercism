use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    ptr: usize, //Marks the starting point for the key. Incremented on every use of munge_in_place to make this stateful

}
pub trait MungeOutput: Iterator<Item = u8> + ExactSizeIterator {}
impl<T> MungeOutput for T where T: Iterator<Item = u8> + ExactSizeIterator {}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<K>(key: &'a K) -> Xorcism<'a>
    where
        K: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref(),
            ptr: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let mut ptr = self.ptr;
        for byte in data.iter_mut() {
            *byte ^= self.key[ptr];
            inc_ptr_wrapping(&mut ptr, self.key.len());
        }
        inc_ptr_wrapping(&mut self.ptr, self.key.len());
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, I, T>(&mut self, data: I) -> impl MungeOutput + 'b
        where I: IntoIterator<Item=T> + ExactSizeIterator + 'b,
              T: Borrow<u8> + 'b,
              'a: 'b

    {
        let mut ptr = self.ptr;
        inc_ptr_wrapping(&mut self.ptr, self.key.len());
        data.into_iter().map(|b| *b.borrow() ^ self.key[ptr])



    }
}

fn inc_ptr_wrapping(ptr: &mut usize, wrap_at: usize) {
    *ptr = (*ptr + 1) % wrap_at;
}
