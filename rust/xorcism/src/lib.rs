/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8]
}

impl<'a> Xorcism<'a> 
{
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<K>(key: &'a K) -> Xorcism<'a> 
        where K: AsRef<[u8]> + 'a,
    {
        Self {
            key: key.as_ref()
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let k = self.key.as_ref();
        let mut k = k.repeat(data.len() / k.len());
        for (key_byte, data_byte) in k.iter().zip(data.iter_mut()) {
            *data_byte ^= *key_byte;
        }

        let first = k.first().cloned().expect("Key is empty");
        for i in 0..k.len() - 1 {
            k[i] = k[i + 1];
        }
        *k.last_mut().expect("Key is empty") = first;
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> {
        unimplemented!();
        // this empty iterator silences a compiler complaint that
        // () doesn't implement ExactSizeIterator
        std::iter::empty()
    }
}
