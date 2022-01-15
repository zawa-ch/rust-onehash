#[cfg(test)] mod tests;
use std::convert::TryFrom;
use crate::HashFunction;

/// MD2のダイジェスト値の型。
pub type MD2Digest = [u8; 16];

/// MD2のSテーブル。
const MD2_S_TABLE: [u8; 256] = [
     41, 46, 67,201,162,216,124,  1, 61, 54, 84,161,236,240,  6, 19,
     98,167,  5,243,192,199,115,140,152,147, 43,217,188, 76,130,202,
     30,155, 87, 60,253,212,224, 22,103, 66,111, 24,138, 23,229, 18,
    190, 78,196,214,218,158,222, 73,160,251,245,142,187, 47,238,122,
    169,104,121,145, 21,178,  7, 63,148,194, 16,137, 11, 34, 95, 33,
    128,127, 93,154, 90,144, 50, 39, 53, 62,204,231,191,247,151,  3,
    255, 25, 48,179, 72,165,181,209,215, 94,146, 42,172, 86,170,198,
     79,184, 56,210,150,164,125,182,118,252,107,226,156,116,  4,241,
     69,157,112, 89,100,113,135, 32,134, 91,207,101,230, 45,168,  2,
     27, 96, 37,173,174,176,185,246, 28, 70, 97,105, 52, 64,126, 15,
     85, 71,163, 35,221, 81,175, 58,195, 92,249,206,186,197,234, 38,
     44, 83, 13,110,133, 40,132,  9,211,223,205,244, 65,129, 77, 82,
    106,220, 55,200,108,193,171,250, 36,225,123,  8, 12,189,177, 74,
    120,136,149,139,227, 99,232,109,233,203,213,254, 59,  0, 29, 57,
    242,239,183, 14,102, 88,208,228,166,119,114,248,235,117, 75, 10,
     49, 68, 80,180,143,237, 31, 26,219,153,141, 51,159, 17,131, 20
];

/// MD2のダイジェストを計算するハッシュ関数。
pub struct MD2HushFunction {
    buffer: MD2Buffer,
    checksum: MD2DataChecksum,
    cache: MD2Chunk,
    seq: usize
}
impl HashFunction for MD2HushFunction {
    type DigestType = MD2Digest;
    fn new() -> Self {
        MD2HushFunction{
            buffer: MD2Buffer::default(),
            checksum: MD2DataChecksum::default(),
            cache: MD2Chunk::default(),
            seq: 0
        }
    }
    fn put_value(&mut self, value: &u8) {
        self.cache[self.seq % 16] = *value;
        self.seq += 1;
        if (self.seq % 16) == 0 {
            self.checksum.update(&self.cache);
            self.buffer.update(&self.cache);
            self.cache = MD2Chunk::default();
        }
    }
    fn digest(&self) -> <Self as HashFunction>::DigestType {
        let mut chunk = self.cache;
        let l: u8 = 16 - u8::try_from(self.seq % 16).unwrap();
        for i in (self.seq % 16)..16 { chunk[i] = l; }
        let mut checksum = self.checksum;
        let mut w = self.buffer;
        checksum.update(&chunk);
        w.update(&chunk);
        let chunk = checksum.into();
        w.update(&chunk);
        w.into()
    }
}

/// MD2ハッシュ計算時のデータチャンク。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MD2Chunk([u8; 16]);
impl MD2Chunk {
}
impl Default for MD2Chunk {
    fn default() -> Self { Self([0; 16]) }
}
impl std::ops::Index<usize> for MD2Chunk {
    type Output = u8;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD2Chunk {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}

/// MD2ハッシュ計算時のデータのチェックサム。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MD2DataChecksum([u8; 16]);
impl MD2DataChecksum {
    fn update(&mut self, chunk_data: &MD2Chunk) {
        let mut t = self.0[15];
        for i in 0..16 {
            self.0[i] ^= MD2_S_TABLE[usize::from(chunk_data[i] ^ t)];
            t = self.0[i];
        }
    }
}
impl Default for MD2DataChecksum {
    fn default() -> Self { Self([0; 16]) }
}
impl std::convert::Into<MD2Chunk> for MD2DataChecksum {
    fn into(self) -> MD2Chunk { MD2Chunk(self.0) }
}

/// MD2の計算バッファ。
#[derive(Clone, Copy)]
struct MD2Buffer([u8; 16]);
impl MD2Buffer {
    fn update(&mut self, chunk_data: &MD2Chunk) {
        let mut buffer_a: [u8; 16] = chunk_data.0;
        let mut buffer_b: [u8; 16] = buffer_a;
        for i in 0..16 { buffer_b[i] ^= self.0[i]; }
        let mut t: u8 = 0;
        for n in 0..18 {
            for i in 0..16 {
                self.0[i] ^= MD2_S_TABLE[usize::from(t)];
                t = self.0[i];
            }
            for i in 0..16 {
                buffer_a[i] ^= MD2_S_TABLE[usize::from(t)];
                t = buffer_a[i];
            }
            for i in 0..16 {
                buffer_b[i] ^= MD2_S_TABLE[usize::from(t)];
                t = buffer_b[i];
            }
            t = t.wrapping_add(n);
        }
    }
}
impl Default for MD2Buffer {
    fn default() -> Self { MD2Buffer([0; 16]) }
}
impl std::convert::Into<MD2Digest> for MD2Buffer {
    fn into(self) -> MD2Digest { self.0 }
}
