#[cfg(test)] mod test;
use crate::HashFunction;
use crate::keccak::*;

#[derive(Clone, Copy, Debug)]
struct SHA3Hash512Chunk([u8; 72]);
impl SHA3Hash512Chunk {
    pub fn get_byte(&self, index: usize) -> u8 { assert!(index < 72); self.0[index] }
    pub fn set_byte(&mut self, index: usize, value: u8) { assert!(index < 72); self.0[index] = value }
}
impl Default for SHA3Hash512Chunk {
    fn default() -> Self { Self([0; 72]) }
}
impl KeccakSpongeInputArray for SHA3Hash512Chunk {
    const R: usize = 576;
    const GETTER: KeccakDataGetter<Self> = KeccakDataGetter::Byte(Self::get_byte, 72);
    const SETTER: KeccakDataSetter<Self> = KeccakDataSetter::Byte(Self::set_byte, 72);
    fn get_bit(&self, index: usize) -> bool { assert!(index < 576); ((self.0[index / 8] >> (index % 8)) & 1) != 0 }
    fn set_bit(&mut self, index: usize, value: bool) { assert!(index < 576); self.0[index / 8] = (self.0[index / 8] & !(1 << (index % 8))) | (if value { 1 } else { 0 } << (index % 8)) }
}

type SHA3Hash512Digest = [u8; 64];

pub struct SHA3Hash512 {
    length: usize,
    chunk: SHA3Hash512Chunk,
    buffer: KeccakSponge<KeccakWord64>
}
impl SHA3Hash512 {
    fn append_bit(o: (&mut SHA3Hash512Chunk, &mut usize, &mut KeccakSponge<KeccakWord64>), value: bool) {
        o.0.set_bit(*o.1, value);
        *o.1 += 1;
        if 576 <= *o.1 {
            o.2.absorb(o.0);
            *o.0 = SHA3Hash512Chunk::default();
            *o.1 -= 576;
        }
    }
}
impl HashFunction for SHA3Hash512 {
    type DigestType = SHA3Hash512Digest;
    fn new() -> Self {
        Self {
            length: 0,
            chunk: SHA3Hash512Chunk::default(),
            buffer: KeccakSponge::<KeccakWord64>::default()
        }
    }
    fn put_value(&mut self, value: &u8) {
        self.chunk.0[(self.length / 8) % 72] = *value;
        self.length += 8;
        if (self.length % 576) == 0 {
            self.buffer.absorb(&self.chunk);
            self.chunk = SHA3Hash512Chunk::default();
            self.length = 0;
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
        if (l % 576) == 0 {
            buf.absorb(&chunk);
            chunk = SHA3Hash512Chunk::default();
        }
        chunk.set_bit(575, true);
        buf.absorb(&chunk);
        //  output
        let mut result: SHA3Hash512Digest = [0; 64];
        chunk = buf.squeeze();
        for i in 0..64 { result[i] = chunk.get_byte(i) }
        result
    }
}
