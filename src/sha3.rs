#[cfg(test)] mod test512;
use crate::HashFunction;
use crate::keccak::*;

#[derive(Clone, Copy, Debug)]
struct SHA3HashChunk<const RSIZE: usize>([u8; RSIZE]);
impl<const RSIZE: usize> SHA3HashChunk<RSIZE> {
    pub fn get_byte(&self, index: usize) -> u8 { assert!(index < RSIZE); self.0[index] }
    pub fn set_byte(&mut self, index: usize, value: u8) { assert!(index < RSIZE); self.0[index] = value }
}
impl<const RSIZE: usize> Default for SHA3HashChunk<RSIZE> {
    fn default() -> Self { Self([0; RSIZE]) }
}
impl<const RSIZE: usize> KeccakSpongeInputArray for SHA3HashChunk<RSIZE> {
    const R: usize = RSIZE * 8;
    const GETTER: KeccakDataGetter<Self> = KeccakDataGetter::Byte(Self::get_byte, RSIZE);
    const SETTER: KeccakDataSetter<Self> = KeccakDataSetter::Byte(Self::set_byte, RSIZE);
    fn get_bit(&self, index: usize) -> bool { assert!(index < Self::R); ((self.0[index / 8] >> (index % 8)) & 1) != 0 }
    fn set_bit(&mut self, index: usize, value: bool) { assert!(index < Self::R); self.0[index / 8] = (self.0[index / 8] & !(1 << (index % 8))) | (if value { 1 } else { 0 } << (index % 8)) }
}

type SHA3HashDigest<const D: usize> = [u8; D];

pub struct SHA3Hash<const D: usize, const R: usize> {
    length: usize,
    chunk: SHA3HashChunk<R>,
    buffer: KeccakSponge<KeccakWord64>
}
impl<const D: usize, const R: usize> SHA3Hash<D, R> {
    fn append_bit(o: (&mut SHA3HashChunk<R>, &mut usize, &mut KeccakSponge<KeccakWord64>), value: bool) {
        o.0.set_bit(*o.1, value);
        *o.1 += 1;
        if SHA3HashChunk::<R>::R <= *o.1 {
            o.2.absorb(o.0);
            *o.0 = SHA3HashChunk::default();
            *o.1 -= SHA3HashChunk::<R>::R;
        }
    }
}
impl<const D: usize, const R: usize> HashFunction for SHA3Hash<D, R> {
    type DigestType = SHA3HashDigest<D>;
    fn new() -> Self {
        Self {
            length: 0,
            chunk: SHA3HashChunk::default(),
            buffer: KeccakSponge::<KeccakWord64>::default()
        }
    }
    fn put_value(&mut self, value: &u8) {
        assert!(self.length < SHA3HashChunk::<R>::R);
        self.chunk.0[self.length / 8] = *value;
        self.length += 8;
        if self.length >= SHA3HashChunk::<R>::R {
            self.buffer.absorb(&self.chunk);
            self.chunk = SHA3HashChunk::default();
            self.length -= SHA3HashChunk::<R>::R;
        }
    }
    fn digest(&self) -> <Self as HashFunction>::DigestType {
        let mut chunk = self.chunk;
        let mut l = self.length;
        let mut buf = self.buffer;
        //  append bit'01'
        Self::append_bit((&mut chunk, &mut l, &mut buf), false);
        Self::append_bit((&mut chunk, &mut l, &mut buf), true);
        //  padding '10...01'
        chunk.set_bit(l, true);
        l += 1;
        if (l % SHA3HashChunk::<R>::R) == 0 {
            buf.absorb(&chunk);
            chunk = SHA3HashChunk::default();
        }
        chunk.set_bit(SHA3HashChunk::<R>::R - 1, true);
        buf.absorb(&chunk);
        //  output
        let mut result: SHA3HashDigest<D> = [0; D];
        chunk = buf.squeeze();
        for i in 0..D { result[i] = chunk.get_byte(i) }
        result
    }
}

pub type SHA3Hash512Digest = SHA3HashDigest<64>;
pub type SHA3Hash512 = SHA3Hash<64, 72>;
