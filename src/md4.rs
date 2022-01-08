use core::fmt::Debug;
use std::convert::{TryFrom, TryInto};
use crate::HashFunction;

/// MD4のダイジェスト値の型。
pub type MD4Digest = [u8; 16];

/// MD4のダイジェストを計算するハッシュ関数。
pub struct MD4HushFunction {
    buffer: MD4Buffer,
    message_length: u64,
    word_cache: MD4Word,
    chunk_cache: MD4Chunk
}
impl HashFunction for MD4HushFunction {
    type DigestType = MD4Digest;
    fn new() -> Self {
        MD4HushFunction{
            buffer: MD4Buffer::default(),
            message_length: 0,
            word_cache: MD4Word::default(),
            chunk_cache: MD4Chunk::default()
        }
    }
    fn put_value(&mut self, value: &u8) {
        self.word_cache[((self.message_length / 8) % 4).try_into().unwrap()] = *value;
        self.message_length += 8;
        if ((self.message_length / 8) % 4) == 0 {
            self.chunk_cache[(((self.message_length / 32) - 1) % 16).try_into().unwrap()] = self.word_cache;
            self.word_cache = MD4Word::default();
            if ((self.message_length / 32) % 16) == 0 {
                round_f(&mut self.buffer, &self.chunk_cache);
                self.chunk_cache = MD4Chunk::default();
            }
        }
    }
    fn digest(&self) -> <Self as HashFunction>::DigestType {
        let mut w = self.buffer;
        let mut chunk = self.chunk_cache;
        chunk[((self.message_length / 32) % 16).try_into().unwrap()] = self.word_cache;
        if (self.message_length % 512) != 0 {
            chunk.set_bit((self.message_length % 512).try_into().unwrap(), true);
        }
        if (self.message_length % 512) >= 448 {
            round_f(&mut w, &chunk);
            chunk = MD4Chunk::default();
        }
        if (self.message_length % 512) == 0 {
            chunk.set_bit((self.message_length % 512).try_into().unwrap(), true);
        }
        chunk[14] = u32::try_from(self.message_length & 0xFFFFFFFF).unwrap().into();
        chunk[15] = u32::try_from((self.message_length >> 32) & 0xFFFFFFFF).unwrap().into();
        round_f(&mut w, &chunk);
        w.into()
    }
}

fn round_f(buffer: &mut MD4Buffer, chunk_data: &MD4Chunk) {
    let mut w = *buffer;
    {
        let shift_table = [ 3, 7, 11, 19 ];
        let index_table = [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];
        for i in 0..16 {
            let f = w[0] + fn1(w[1], w[2], w[3]) + chunk_data[index_table[i]];
            w[0] = w[3];
            w[3] = w[2];
            w[2] = w[1];
            w[1] = f << shift_table[i % 4];
        }
    }
    {
        let shift_table = [ 3, 5, 9, 13 ];
        let index_table = [ 0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15 ];
        const MD4SQRT2: u32 = 0x5A827999;
        for i in 0..16 {
            let f = w[0] + fn2(w[1], w[2], w[3]) + chunk_data[index_table[i]] + MD4Word::from(MD4SQRT2);
            w[0] = w[3];
            w[3] = w[2];
            w[2] = w[1];
            w[1] = f << shift_table[i % 4];
        }
    }
    {
        let shift_table = [ 3, 9, 11, 15 ];
        let index_table = [ 0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15 ];
        const MD4SQRT3: u32 = 0x6ED9EBA1;
        for i in 0..16 {
            let f = w[0] + fn3(w[1], w[2], w[3]) + chunk_data[index_table[i]] + MD4Word::from(MD4SQRT3);
            w[0] = w[3];
            w[3] = w[2];
            w[2] = w[1];
            w[1] = f << shift_table[i % 4];
        }
    }
    for i in 0..4 { buffer[i] += w[i]; }
}
fn fn1(x: MD4Word, y: MD4Word, z: MD4Word) -> MD4Word { (x & y) | (!x & z) }
fn fn2(x: MD4Word, y: MD4Word, z: MD4Word) -> MD4Word { (x & y) ^ (x & z) ^ (y & z) }
fn fn3(x: MD4Word, y: MD4Word, z: MD4Word) -> MD4Word { x ^ y ^ z }

/// MD4計算に用いる8ビットのバイト値。
struct MD4Byte {}
impl MD4Byte {
    fn set_bit(obj: &mut u8, index: usize, value: bool) {
        assert!(index < 8);
        *obj = (*obj & !(0x80 >> (index % 8))) | ( if value { 0x80 } else { 0 } >> (index % 8))
    }
}
#[cfg(test)]
mod md4byte_test {
    use crate::md4::MD4Byte;
    #[test] fn set_bit0_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 0, true); assert_eq!(b, 0x80); }
    #[test] fn set_bit1_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 1, true); assert_eq!(b, 0x40); }
    #[test] fn set_bit2_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 2, true); assert_eq!(b, 0x20); }
    #[test] fn set_bit3_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 3, true); assert_eq!(b, 0x10); }
    #[test] fn set_bit4_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 4, true); assert_eq!(b, 0x08); }
    #[test] fn set_bit5_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 5, true); assert_eq!(b, 0x04); }
    #[test] fn set_bit6_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 6, true); assert_eq!(b, 0x02); }
    #[test] fn set_bit7_high() { let mut b: u8 = 0x00; MD4Byte::set_bit(&mut b, 7, true); assert_eq!(b, 0x01); }
    #[test] fn set_bit0_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 0, false); assert_eq!(b, 0x7f); }
    #[test] fn set_bit1_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 1, false); assert_eq!(b, 0xbf); }
    #[test] fn set_bit2_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 2, false); assert_eq!(b, 0xdf); }
    #[test] fn set_bit3_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 3, false); assert_eq!(b, 0xef); }
    #[test] fn set_bit4_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 4, false); assert_eq!(b, 0xf7); }
    #[test] fn set_bit5_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 5, false); assert_eq!(b, 0xfb); }
    #[test] fn set_bit6_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 6, false); assert_eq!(b, 0xfd); }
    #[test] fn set_bit7_low() { let mut b: u8 = 0xFF; MD4Byte::set_bit(&mut b, 7, false); assert_eq!(b, 0xfe); }
}
/// MD4計算に用いる32ビットのワード値。
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MD4Word([u8; 4]);
impl MD4Word {
    fn set_bit(&mut self, index: usize, value: bool) {
        assert!(index < 32);
        MD4Byte::set_bit(&mut self.0[index / 8], index % 8, value)
    }
}
impl Default for MD4Word {
    fn default() -> Self { Self([0; 4]) }
}
impl std::ops::Index<usize> for MD4Word {
    type Output = u8;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD4Word {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}
impl std::ops::Add for MD4Word {
    type Output = Self;
    fn add(self, rhs: Self) -> <Self as std::ops::Add<Self>>::Output { <Self as From<u32>>::from(u32::wrapping_add(<Self as Into<u32>>::into(self), <Self as Into<u32>>::into(rhs))) }
}
impl std::ops::AddAssign for MD4Word {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs }
}
impl std::ops::BitAnd for MD4Word {
    type Output = Self;
    fn bitand(self, rhs: Self) -> <Self as std::ops::BitAnd<Self>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self) & <Self as Into<u32>>::into(rhs)) }
}
impl std::ops::BitOr for MD4Word {
    type Output = Self;
    fn bitor(self, rhs: Self) -> <Self as std::ops::BitOr<Self>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self) | <Self as Into<u32>>::into(rhs)) }
}
impl std::ops::BitXor for MD4Word {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> <Self as std::ops::BitXor<Self>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self) ^ <Self as Into<u32>>::into(rhs)) }
}
impl std::ops::Not for MD4Word {
    type Output = Self;
    fn not(self) -> <Self as std::ops::Not>::Output { <Self as From<u32>>::from(!<Self as Into<u32>>::into(self)) }
}
impl std::ops::Shl<u32> for MD4Word {
    type Output = Self;
    fn shl(self, rhs: u32) -> <Self as std::ops::Shl<u32>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self).rotate_left(rhs)) }
}
impl std::ops::Shr<u32> for MD4Word {
    type Output = Self;
    fn shr(self, rhs: u32) -> <Self as std::ops::Shr<u32>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self).rotate_right(rhs)) }
}
impl std::ops::BitAndAssign for MD4Word {
    fn bitand_assign(&mut self, rhs: Self) { *self = *self & rhs }
}
impl std::ops::BitOrAssign for MD4Word {
    fn bitor_assign(&mut self, rhs: Self) { *self = *self | rhs }
}
impl std::ops::BitXorAssign for MD4Word {
    fn bitxor_assign(&mut self, rhs: Self) { *self = *self ^ rhs }
}
impl From<u32> for MD4Word {
    fn from(value: u32) -> Self { Self(value.to_le_bytes()) }
}
impl Into<u32> for MD4Word {
    fn into(self) -> u32 { u32::from_le_bytes(self.0) }
}
impl Debug for MD4Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("MD4Word({})", <Self as Into<u32>>::into(*self)))
    }
}
#[cfg(test)]
mod md4word_test {
    use crate::md4::MD4Word;
    #[test] fn from_pattern1() { let w = 0x67452301; assert_eq!(<MD4Word as From<u32>>::from(w), MD4Word([0x01, 0x23, 0x45, 0x67])) }
    #[test] fn from_pattern2() { let w = 0xefcdab89; assert_eq!(<MD4Word as From<u32>>::from(w), MD4Word([0x89, 0xab, 0xcd, 0xef])) }
    #[test] fn from_pattern3() { let w = 0x98badcfe; assert_eq!(<MD4Word as From<u32>>::from(w), MD4Word([0xfe, 0xdc, 0xba, 0x98])) }
    #[test] fn from_pattern4() { let w = 0x10325476; assert_eq!(<MD4Word as From<u32>>::from(w), MD4Word([0x76, 0x54, 0x32, 0x10])) }
    #[test] fn into_pattern1() { let w = MD4Word([0x01, 0x23, 0x45, 0x67]); assert_eq!(<MD4Word as Into<u32>>::into(w), 0x67452301) }
    #[test] fn into_pattern2() { let w = MD4Word([0x89, 0xab, 0xcd, 0xef]); assert_eq!(<MD4Word as Into<u32>>::into(w), 0xefcdab89) }
    #[test] fn into_pattern3() { let w = MD4Word([0xfe, 0xdc, 0xba, 0x98]); assert_eq!(<MD4Word as Into<u32>>::into(w), 0x98badcfe) }
    #[test] fn into_pattern4() { let w = MD4Word([0x76, 0x54, 0x32, 0x10]); assert_eq!(<MD4Word as Into<u32>>::into(w), 0x10325476) }
    #[test] fn set_bit0_high() { let mut b = MD4Word::default(); b.set_bit(0, true); assert_eq!(b, MD4Word([0x80, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit1_high() { let mut b = MD4Word::default(); b.set_bit(1, true); assert_eq!(b, MD4Word([0x40, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit2_high() { let mut b = MD4Word::default(); b.set_bit(2, true); assert_eq!(b, MD4Word([0x20, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit3_high() { let mut b = MD4Word::default(); b.set_bit(3, true); assert_eq!(b, MD4Word([0x10, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit4_high() { let mut b = MD4Word::default(); b.set_bit(4, true); assert_eq!(b, MD4Word([0x08, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit5_high() { let mut b = MD4Word::default(); b.set_bit(5, true); assert_eq!(b, MD4Word([0x04, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit6_high() { let mut b = MD4Word::default(); b.set_bit(6, true); assert_eq!(b, MD4Word([0x02, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit7_high() { let mut b = MD4Word::default(); b.set_bit(7, true); assert_eq!(b, MD4Word([0x01, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit8_high() { let mut b = MD4Word::default(); b.set_bit(8, true); assert_eq!(b, MD4Word([0x00, 0x80, 0x00, 0x00])); }
    #[test] fn set_bit9_high() { let mut b = MD4Word::default(); b.set_bit(9, true); assert_eq!(b, MD4Word([0x00, 0x40, 0x00, 0x00])); }
    #[test] fn set_bit10_high() { let mut b = MD4Word::default(); b.set_bit(10, true); assert_eq!(b, MD4Word([0x00, 0x20, 0x00, 0x00])); }
    #[test] fn set_bit11_high() { let mut b = MD4Word::default(); b.set_bit(11, true); assert_eq!(b, MD4Word([0x00, 0x10, 0x00, 0x00])); }
    #[test] fn set_bit12_high() { let mut b = MD4Word::default(); b.set_bit(12, true); assert_eq!(b, MD4Word([0x00, 0x08, 0x00, 0x00])); }
    #[test] fn set_bit13_high() { let mut b = MD4Word::default(); b.set_bit(13, true); assert_eq!(b, MD4Word([0x00, 0x04, 0x00, 0x00])); }
    #[test] fn set_bit14_high() { let mut b = MD4Word::default(); b.set_bit(14, true); assert_eq!(b, MD4Word([0x00, 0x02, 0x00, 0x00])); }
    #[test] fn set_bit15_high() { let mut b = MD4Word::default(); b.set_bit(15, true); assert_eq!(b, MD4Word([0x00, 0x01, 0x00, 0x00])); }
    #[test] fn set_bit16_high() { let mut b = MD4Word::default(); b.set_bit(16, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x80, 0x00])); }
    #[test] fn set_bit17_high() { let mut b = MD4Word::default(); b.set_bit(17, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x40, 0x00])); }
    #[test] fn set_bit18_high() { let mut b = MD4Word::default(); b.set_bit(18, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x20, 0x00])); }
    #[test] fn set_bit19_high() { let mut b = MD4Word::default(); b.set_bit(19, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x10, 0x00])); }
    #[test] fn set_bit20_high() { let mut b = MD4Word::default(); b.set_bit(20, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x08, 0x00])); }
    #[test] fn set_bit21_high() { let mut b = MD4Word::default(); b.set_bit(21, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x04, 0x00])); }
    #[test] fn set_bit22_high() { let mut b = MD4Word::default(); b.set_bit(22, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x02, 0x00])); }
    #[test] fn set_bit23_high() { let mut b = MD4Word::default(); b.set_bit(23, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x01, 0x00])); }
    #[test] fn set_bit24_high() { let mut b = MD4Word::default(); b.set_bit(24, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x80])); }
    #[test] fn set_bit25_high() { let mut b = MD4Word::default(); b.set_bit(25, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x40])); }
    #[test] fn set_bit26_high() { let mut b = MD4Word::default(); b.set_bit(26, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x20])); }
    #[test] fn set_bit27_high() { let mut b = MD4Word::default(); b.set_bit(27, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x10])); }
    #[test] fn set_bit28_high() { let mut b = MD4Word::default(); b.set_bit(28, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x08])); }
    #[test] fn set_bit29_high() { let mut b = MD4Word::default(); b.set_bit(29, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x04])); }
    #[test] fn set_bit30_high() { let mut b = MD4Word::default(); b.set_bit(30, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x02])); }
    #[test] fn set_bit31_high() { let mut b = MD4Word::default(); b.set_bit(31, true); assert_eq!(b, MD4Word([0x00, 0x00, 0x00, 0x01])); }
    #[test] fn set_bit0_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(0, false); assert_eq!(b, MD4Word([0x7f, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit1_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(1, false); assert_eq!(b, MD4Word([0xbf, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit2_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(2, false); assert_eq!(b, MD4Word([0xdf, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit3_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(3, false); assert_eq!(b, MD4Word([0xef, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit4_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(4, false); assert_eq!(b, MD4Word([0xf7, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit5_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(5, false); assert_eq!(b, MD4Word([0xfb, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit6_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(6, false); assert_eq!(b, MD4Word([0xfd, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit7_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(7, false); assert_eq!(b, MD4Word([0xfe, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit8_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(8, false); assert_eq!(b, MD4Word([0xff, 0x7f, 0xff, 0xff])); }
    #[test] fn set_bit9_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(9, false); assert_eq!(b, MD4Word([0xff, 0xbf, 0xff, 0xff])); }
    #[test] fn set_bit10_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(10, false); assert_eq!(b, MD4Word([0xff, 0xdf, 0xff, 0xff])); }
    #[test] fn set_bit11_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(11, false); assert_eq!(b, MD4Word([0xff, 0xef, 0xff, 0xff])); }
    #[test] fn set_bit12_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(12, false); assert_eq!(b, MD4Word([0xff, 0xf7, 0xff, 0xff])); }
    #[test] fn set_bit13_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(13, false); assert_eq!(b, MD4Word([0xff, 0xfb, 0xff, 0xff])); }
    #[test] fn set_bit14_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(14, false); assert_eq!(b, MD4Word([0xff, 0xfd, 0xff, 0xff])); }
    #[test] fn set_bit15_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(15, false); assert_eq!(b, MD4Word([0xff, 0xfe, 0xff, 0xff])); }
    #[test] fn set_bit16_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(16, false); assert_eq!(b, MD4Word([0xff, 0xff, 0x7f, 0xff])); }
    #[test] fn set_bit17_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(17, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xbf, 0xff])); }
    #[test] fn set_bit18_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(18, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xdf, 0xff])); }
    #[test] fn set_bit19_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(19, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xef, 0xff])); }
    #[test] fn set_bit20_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(20, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xf7, 0xff])); }
    #[test] fn set_bit21_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(21, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xfb, 0xff])); }
    #[test] fn set_bit22_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(22, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xfd, 0xff])); }
    #[test] fn set_bit23_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(23, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xfe, 0xff])); }
    #[test] fn set_bit24_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(24, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0x7f])); }
    #[test] fn set_bit25_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(25, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xbf])); }
    #[test] fn set_bit26_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(26, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xdf])); }
    #[test] fn set_bit27_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(27, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xef])); }
    #[test] fn set_bit28_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(28, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xf7])); }
    #[test] fn set_bit29_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(29, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xfb])); }
    #[test] fn set_bit30_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(30, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xfd])); }
    #[test] fn set_bit31_low() { let mut b = MD4Word::from(0xFFFFFFFF); b.set_bit(31, false); assert_eq!(b, MD4Word([0xff, 0xff, 0xff, 0xfe])); }
    #[test] fn and_pattern1() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) & MD4Word([0xfe, 0xdc, 0xba, 0x98]), MD4Word([0x00, 0x00, 0x00, 0x00])) }
    #[test] fn and_pattern2() { assert_eq!(MD4Word([0x89, 0xab, 0xcd, 0xef]) & MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0x00, 0x00, 0x00, 0x00])) }
    #[test] fn and_pattern3() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) & MD4Word([0x89, 0xab, 0xcd, 0xef]), MD4Word([0x01, 0x23, 0x45, 0x67])) }
    #[test] fn and_pattern4() { assert_eq!(MD4Word([0xfe, 0xdc, 0xba, 0x98]) & MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0x76, 0x54, 0x32, 0x10])) }
    #[test] fn or_pattern1() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) | MD4Word([0xfe, 0xdc, 0xba, 0x98]), MD4Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn or_pattern2() { assert_eq!(MD4Word([0x89, 0xab, 0xcd, 0xef]) | MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn or_pattern3() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) | MD4Word([0x89, 0xab, 0xcd, 0xef]), MD4Word([0x89, 0xab, 0xcd, 0xef])) }
    #[test] fn or_pattern4() { assert_eq!(MD4Word([0xfe, 0xdc, 0xba, 0x98]) | MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0xfe, 0xdc, 0xba, 0x98])) }
    #[test] fn xor_pattern1() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) ^ MD4Word([0xfe, 0xdc, 0xba, 0x98]), MD4Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn xor_pattern2() { assert_eq!(MD4Word([0x89, 0xab, 0xcd, 0xef]) ^ MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn xor_pattern3() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) ^ MD4Word([0x89, 0xab, 0xcd, 0xef]), MD4Word([0x88, 0x88, 0x88, 0x88])) }
    #[test] fn xor_pattern4() { assert_eq!(MD4Word([0xfe, 0xdc, 0xba, 0x98]) ^ MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0x88, 0x88, 0x88, 0x88])) }
    #[test] fn not_pattern1() { assert_eq!(!MD4Word([0x01, 0x23, 0x45, 0x67]), MD4Word([0xfe, 0xdc, 0xba, 0x98])) }
    #[test] fn not_pattern2() { assert_eq!(!MD4Word([0x89, 0xab, 0xcd, 0xef]), MD4Word([0x76, 0x54, 0x32, 0x10])) }
    #[test] fn not_pattern3() { assert_eq!(!MD4Word([0xfe, 0xdc, 0xba, 0x98]), MD4Word([0x01, 0x23, 0x45, 0x67])) }
    #[test] fn not_pattern4() { assert_eq!(!MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0x89, 0xab, 0xcd, 0xef])) }
    #[test] fn add_pattern1() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) + MD4Word([0xfe, 0xdc, 0xba, 0x98]), MD4Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn add_pattern2() { assert_eq!(MD4Word([0x89, 0xab, 0xcd, 0xef]) + MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn add_pattern3() { assert_eq!(MD4Word([0x01, 0x23, 0x45, 0x67]) + MD4Word([0x89, 0xab, 0xcd, 0xef]), MD4Word([0x8a, 0xce, 0x12, 0x57])) }
    #[test] fn add_pattern4() { assert_eq!(MD4Word([0xfe, 0xdc, 0xba, 0x98]) + MD4Word([0x76, 0x54, 0x32, 0x10]), MD4Word([0x74, 0x31, 0xed, 0xa8])) }
}
/// MD4ハッシュ計算時のデータチャンク。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MD4Chunk([MD4Word; 16]);
impl MD4Chunk {
    fn set_bit(&mut self, index: usize, value: bool) {
        assert!(index < 512);
        self[index / 32].set_bit(index % 32, value)
    }
}
impl Default for MD4Chunk {
    fn default() -> Self { Self([MD4Word::default(); 16]) }
}
impl std::ops::Index<usize> for MD4Chunk {
    type Output = MD4Word;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD4Chunk {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}
#[cfg(test)]
mod md4chunk_test {
    use crate::md4::{MD4Word, MD4Chunk};
    #[test] fn set_bit0_high() { let mut b = MD4Chunk::default(); b.set_bit(0, true); assert_eq!(b[0], MD4Word([0x80, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit16_high() { let mut b = MD4Chunk::default(); b.set_bit(16, true); assert_eq!(b[0], MD4Word([0x00, 0x00, 0x80, 0x00])); }
    #[test] fn set_bit33_high() { let mut b = MD4Chunk::default(); b.set_bit(33, true); assert_eq!(b[1], MD4Word([0x40, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit49_high() { let mut b = MD4Chunk::default(); b.set_bit(49, true); assert_eq!(b[1], MD4Word([0x00, 0x00, 0x40, 0x00])); }
    #[test] fn set_bit66_high() { let mut b = MD4Chunk::default(); b.set_bit(66, true); assert_eq!(b[2], MD4Word([0x20, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit82_high() { let mut b = MD4Chunk::default(); b.set_bit(82, true); assert_eq!(b[2], MD4Word([0x00, 0x00, 0x20, 0x00])); }
    #[test] fn set_bit99_high() { let mut b = MD4Chunk::default(); b.set_bit(99, true); assert_eq!(b[3], MD4Word([0x10, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit115_high() { let mut b = MD4Chunk::default(); b.set_bit(115, true); assert_eq!(b[3], MD4Word([0x00, 0x00, 0x10, 0x00])); }
    #[test] fn set_bit132_high() { let mut b = MD4Chunk::default(); b.set_bit(132, true); assert_eq!(b[4], MD4Word([0x08, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit148_high() { let mut b = MD4Chunk::default(); b.set_bit(148, true); assert_eq!(b[4], MD4Word([0x00, 0x00, 0x08, 0x00])); }
    #[test] fn set_bit165_high() { let mut b = MD4Chunk::default(); b.set_bit(165, true); assert_eq!(b[5], MD4Word([0x04, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit181_high() { let mut b = MD4Chunk::default(); b.set_bit(181, true); assert_eq!(b[5], MD4Word([0x00, 0x00, 0x04, 0x00])); }
    #[test] fn set_bit198_high() { let mut b = MD4Chunk::default(); b.set_bit(198, true); assert_eq!(b[6], MD4Word([0x02, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit214_high() { let mut b = MD4Chunk::default(); b.set_bit(214, true); assert_eq!(b[6], MD4Word([0x00, 0x00, 0x02, 0x00])); }
    #[test] fn set_bit231_high() { let mut b = MD4Chunk::default(); b.set_bit(231, true); assert_eq!(b[7], MD4Word([0x01, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit247_high() { let mut b = MD4Chunk::default(); b.set_bit(247, true); assert_eq!(b[7], MD4Word([0x00, 0x00, 0x01, 0x00])); }
    #[test] fn set_bit264_high() { let mut b = MD4Chunk::default(); b.set_bit(264, true); assert_eq!(b[8], MD4Word([0x00, 0x80, 0x00, 0x00])); }
    #[test] fn set_bit280_high() { let mut b = MD4Chunk::default(); b.set_bit(280, true); assert_eq!(b[8], MD4Word([0x00, 0x00, 0x00, 0x80])); }
    #[test] fn set_bit297_high() { let mut b = MD4Chunk::default(); b.set_bit(297, true); assert_eq!(b[9], MD4Word([0x00, 0x40, 0x00, 0x00])); }
    #[test] fn set_bit313_high() { let mut b = MD4Chunk::default(); b.set_bit(313, true); assert_eq!(b[9], MD4Word([0x00, 0x00, 0x00, 0x40])); }
    #[test] fn set_bit330_high() { let mut b = MD4Chunk::default(); b.set_bit(330, true); assert_eq!(b[10], MD4Word([0x00, 0x20, 0x00, 0x00])); }
    #[test] fn set_bit346_high() { let mut b = MD4Chunk::default(); b.set_bit(346, true); assert_eq!(b[10], MD4Word([0x00, 0x00, 0x00, 0x20])); }
    #[test] fn set_bit363_high() { let mut b = MD4Chunk::default(); b.set_bit(363, true); assert_eq!(b[11], MD4Word([0x00, 0x10, 0x00, 0x00])); }
    #[test] fn set_bit379_high() { let mut b = MD4Chunk::default(); b.set_bit(379, true); assert_eq!(b[11], MD4Word([0x00, 0x00, 0x00, 0x10])); }
    #[test] fn set_bit396_high() { let mut b = MD4Chunk::default(); b.set_bit(396, true); assert_eq!(b[12], MD4Word([0x00, 0x08, 0x00, 0x00])); }
    #[test] fn set_bit412_high() { let mut b = MD4Chunk::default(); b.set_bit(412, true); assert_eq!(b[12], MD4Word([0x00, 0x00, 0x00, 0x08])); }
    #[test] fn set_bit429_high() { let mut b = MD4Chunk::default(); b.set_bit(429, true); assert_eq!(b[13], MD4Word([0x00, 0x04, 0x00, 0x00])); }
    #[test] fn set_bit445_high() { let mut b = MD4Chunk::default(); b.set_bit(445, true); assert_eq!(b[13], MD4Word([0x00, 0x00, 0x00, 0x04])); }
    #[test] fn set_bit462_high() { let mut b = MD4Chunk::default(); b.set_bit(462, true); assert_eq!(b[14], MD4Word([0x00, 0x02, 0x00, 0x00])); }
    #[test] fn set_bit478_high() { let mut b = MD4Chunk::default(); b.set_bit(478, true); assert_eq!(b[14], MD4Word([0x00, 0x00, 0x00, 0x02])); }
    #[test] fn set_bit495_high() { let mut b = MD4Chunk::default(); b.set_bit(495, true); assert_eq!(b[15], MD4Word([0x00, 0x01, 0x00, 0x00])); }
    #[test] fn set_bit511_high() { let mut b = MD4Chunk::default(); b.set_bit(511, true); assert_eq!(b[15], MD4Word([0x00, 0x00, 0x00, 0x01])); }
}
/// MD4の計算バッファ。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MD4Buffer([MD4Word; 4]);
impl MD4Buffer {
    fn at_u8(&self, index: usize) -> u8 { self[index/4][index%4] }
}
impl Default for MD4Buffer {
    fn default() -> Self { MD4Buffer([MD4Word([0x01, 0x23, 0x45, 0x67]), MD4Word([0x89, 0xab, 0xcd, 0xef]), MD4Word([0xfe, 0xdc, 0xba, 0x98]), MD4Word([0x76, 0x54, 0x32, 0x10])]) }
}
impl std::ops::Index<usize> for MD4Buffer {
    type Output = MD4Word;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD4Buffer {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}
impl std::convert::Into<MD4Digest> for MD4Buffer {
    fn into(self) -> MD4Digest {
        let mut result: MD4Digest = [0; 16];
        for i in 0..16 {
            result[i] = self.at_u8(i);
        }
        result
    }
}
#[cfg(test)]
mod md4buffer_test {
    use crate::md4::{MD4Buffer, MD4Digest};
    #[test] fn convert() { assert_eq!(<MD4Buffer as Into<MD4Digest>>::into(MD4Buffer::default()), [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]); }
}
