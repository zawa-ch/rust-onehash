use core::fmt::Debug;
use std::convert::{TryFrom, TryInto};
use crate::HashFunction;

const MD5TABLE: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

/// MD5のダイジェスト値の型。
pub type MD5Digest = [u8; 16];

/// MD5のダイジェストを計算するハッシュ関数。
pub struct MD5HushFunction {
    buffer: MD5Buffer,
    message_length: u64,
    word_cache: MD5Word,
    chunk_cache: MD5Chunk
}
impl HashFunction for MD5HushFunction {
    type DigestType = MD5Digest;
    fn new() -> Self {
        MD5HushFunction{
            buffer: MD5Buffer::default(),
            message_length: 0,
            word_cache: MD5Word::default(),
            chunk_cache: MD5Chunk::default()
        }
    }
    fn put_value(&mut self, value: &u8) {
        self.word_cache[((self.message_length / 8) % 4).try_into().unwrap()] = *value;
        self.message_length += 8;
        if ((self.message_length / 8) % 4) == 0 {
            self.chunk_cache[(((self.message_length / 32) - 1) % 16).try_into().unwrap()] = self.word_cache;
            self.word_cache = MD5Word::default();
            if ((self.message_length / 32) % 16) == 0 {
                round_f(&mut self.buffer, &self.chunk_cache);
                self.chunk_cache = MD5Chunk::default();
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
            chunk = MD5Chunk::default();
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

fn round_f(buffer: &mut MD5Buffer, chunk_data: &MD5Chunk) {
    let mut w = *buffer;
    for i in 0..64 {
        let shift_table: [u32; 4];
        let f: fn(MD5Word, MD5Word, MD5Word) -> MD5Word;
        let data_index: usize;
        if i < 16 {
            shift_table = [ 7, 12, 17, 22 ];
            f = fn1;
            data_index = i;
        }
        else if i < 32 {
            shift_table = [ 5, 9, 14, 20 ];
            f = fn2;
            data_index = (i * 5 + 1) % 16;
        }
        else if i < 48 {
            shift_table = [ 4, 11, 16, 23 ];
            f = fn3;
            data_index = (i * 3 + 5) % 16;
        }
        else {
            shift_table = [ 6, 10, 15, 21 ];
            f = fn4;
            data_index = (i * 7) % 16;
        }
        let f = w[0] + f(w[1], w[2], w[3]) + MD5Word::from(MD5TABLE[i]) + chunk_data[data_index];
        w[0] = w[3];
        w[3] = w[2];
        w[2] = w[1];
        w[1] += f << shift_table[i % 4];
    }
    for i in 0..4 { buffer[i] += w[i]; }
}
fn fn1(x: MD5Word, y: MD5Word, z: MD5Word) -> MD5Word { (x & y) | (!x & z) }
fn fn2(x: MD5Word, y: MD5Word, z: MD5Word) -> MD5Word { (x & z) | (y & !z) }
fn fn3(x: MD5Word, y: MD5Word, z: MD5Word) -> MD5Word { x ^ y ^ z }
fn fn4(x: MD5Word, y: MD5Word, z: MD5Word) -> MD5Word { y ^ (x | !z) }

#[cfg(test)]
mod md5table_test {
    use crate::md5::MD5TABLE;
    #[test] fn table0() { assert_eq!(MD5TABLE[0], 0xd76aa478) }
    #[test] fn table1() { assert_eq!(MD5TABLE[1], 0xe8c7b756) }
    #[test] fn table2() { assert_eq!(MD5TABLE[2], 0x242070db) }
    #[test] fn table3() { assert_eq!(MD5TABLE[3], 0xc1bdceee) }
    #[test] fn table4() { assert_eq!(MD5TABLE[4], 0xf57c0faf) }
    #[test] fn table5() { assert_eq!(MD5TABLE[5], 0x4787c62a) }
    #[test] fn table6() { assert_eq!(MD5TABLE[6], 0xa8304613) }
    #[test] fn table7() { assert_eq!(MD5TABLE[7], 0xfd469501) }
    #[test] fn table8() { assert_eq!(MD5TABLE[8], 0x698098d8) }
    #[test] fn table9() { assert_eq!(MD5TABLE[9], 0x8b44f7af) }
    #[test] fn table10() { assert_eq!(MD5TABLE[10], 0xffff5bb1) }
    #[test] fn table11() { assert_eq!(MD5TABLE[11], 0x895cd7be) }
    #[test] fn table12() { assert_eq!(MD5TABLE[12], 0x6b901122) }
    #[test] fn table13() { assert_eq!(MD5TABLE[13], 0xfd987193) }
    #[test] fn table14() { assert_eq!(MD5TABLE[14], 0xa679438e) }
    #[test] fn table15() { assert_eq!(MD5TABLE[15], 0x49b40821) }
    #[test] fn table16() { assert_eq!(MD5TABLE[16], 0xf61e2562) }
    #[test] fn table17() { assert_eq!(MD5TABLE[17], 0xc040b340) }
    #[test] fn table18() { assert_eq!(MD5TABLE[18], 0x265e5a51) }
    #[test] fn table19() { assert_eq!(MD5TABLE[19], 0xe9b6c7aa) }
    #[test] fn table20() { assert_eq!(MD5TABLE[20], 0xd62f105d) }
    #[test] fn table21() { assert_eq!(MD5TABLE[21], 0x2441453) }
    #[test] fn table22() { assert_eq!(MD5TABLE[22], 0xd8a1e681) }
    #[test] fn table23() { assert_eq!(MD5TABLE[23], 0xe7d3fbc8) }
    #[test] fn table24() { assert_eq!(MD5TABLE[24], 0x21e1cde6) }
    #[test] fn table25() { assert_eq!(MD5TABLE[25], 0xc33707d6) }
    #[test] fn table26() { assert_eq!(MD5TABLE[26], 0xf4d50d87) }
    #[test] fn table27() { assert_eq!(MD5TABLE[27], 0x455a14ed) }
    #[test] fn table28() { assert_eq!(MD5TABLE[28], 0xa9e3e905) }
    #[test] fn table29() { assert_eq!(MD5TABLE[29], 0xfcefa3f8) }
    #[test] fn table30() { assert_eq!(MD5TABLE[30], 0x676f02d9) }
    #[test] fn table31() { assert_eq!(MD5TABLE[31], 0x8d2a4c8a) }
    #[test] fn table32() { assert_eq!(MD5TABLE[32], 0xfffa3942) }
    #[test] fn table33() { assert_eq!(MD5TABLE[33], 0x8771f681) }
    #[test] fn table34() { assert_eq!(MD5TABLE[34], 0x6d9d6122) }
    #[test] fn table35() { assert_eq!(MD5TABLE[35], 0xfde5380c) }
    #[test] fn table36() { assert_eq!(MD5TABLE[36], 0xa4beea44) }
    #[test] fn table37() { assert_eq!(MD5TABLE[37], 0x4bdecfa9) }
    #[test] fn table38() { assert_eq!(MD5TABLE[38], 0xf6bb4b60) }
    #[test] fn table39() { assert_eq!(MD5TABLE[39], 0xbebfbc70) }
    #[test] fn table40() { assert_eq!(MD5TABLE[40], 0x289b7ec6) }
    #[test] fn table41() { assert_eq!(MD5TABLE[41], 0xeaa127fa) }
    #[test] fn table42() { assert_eq!(MD5TABLE[42], 0xd4ef3085) }
    #[test] fn table43() { assert_eq!(MD5TABLE[43], 0x4881d05) }
    #[test] fn table44() { assert_eq!(MD5TABLE[44], 0xd9d4d039) }
    #[test] fn table45() { assert_eq!(MD5TABLE[45], 0xe6db99e5) }
    #[test] fn table46() { assert_eq!(MD5TABLE[46], 0x1fa27cf8) }
    #[test] fn table47() { assert_eq!(MD5TABLE[47], 0xc4ac5665) }
    #[test] fn table48() { assert_eq!(MD5TABLE[48], 0xf4292244) }
    #[test] fn table49() { assert_eq!(MD5TABLE[49], 0x432aff97) }
    #[test] fn table50() { assert_eq!(MD5TABLE[50], 0xab9423a7) }
    #[test] fn table51() { assert_eq!(MD5TABLE[51], 0xfc93a039) }
    #[test] fn table52() { assert_eq!(MD5TABLE[52], 0x655b59c3) }
    #[test] fn table53() { assert_eq!(MD5TABLE[53], 0x8f0ccc92) }
    #[test] fn table54() { assert_eq!(MD5TABLE[54], 0xffeff47d) }
    #[test] fn table55() { assert_eq!(MD5TABLE[55], 0x85845dd1) }
    #[test] fn table56() { assert_eq!(MD5TABLE[56], 0x6fa87e4f) }
    #[test] fn table57() { assert_eq!(MD5TABLE[57], 0xfe2ce6e0) }
    #[test] fn table58() { assert_eq!(MD5TABLE[58], 0xa3014314) }
    #[test] fn table59() { assert_eq!(MD5TABLE[59], 0x4e0811a1) }
    #[test] fn table60() { assert_eq!(MD5TABLE[60], 0xf7537e82) }
    #[test] fn table61() { assert_eq!(MD5TABLE[61], 0xbd3af235) }
    #[test] fn table62() { assert_eq!(MD5TABLE[62], 0x2ad7d2bb) }
    #[test] fn table63() { assert_eq!(MD5TABLE[63], 0xeb86d391) }
}

/// MD5計算に用いる8ビットのバイト値。
struct MD5Byte {}
impl MD5Byte {
    fn set_bit(obj: &mut u8, index: usize, value: bool) {
        assert!(index < 8);
        *obj = (*obj & !(0x80 >> (index % 8))) | ( if value { 0x80 } else { 0 } >> (index % 8))
    }
}
#[cfg(test)]
mod md5byte_test {
    use crate::md5::MD5Byte;
    #[test] fn set_bit0_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 0, true); assert_eq!(b, 0x80); }
    #[test] fn set_bit1_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 1, true); assert_eq!(b, 0x40); }
    #[test] fn set_bit2_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 2, true); assert_eq!(b, 0x20); }
    #[test] fn set_bit3_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 3, true); assert_eq!(b, 0x10); }
    #[test] fn set_bit4_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 4, true); assert_eq!(b, 0x08); }
    #[test] fn set_bit5_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 5, true); assert_eq!(b, 0x04); }
    #[test] fn set_bit6_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 6, true); assert_eq!(b, 0x02); }
    #[test] fn set_bit7_high() { let mut b: u8 = 0x00; MD5Byte::set_bit(&mut b, 7, true); assert_eq!(b, 0x01); }
    #[test] fn set_bit0_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 0, false); assert_eq!(b, 0x7f); }
    #[test] fn set_bit1_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 1, false); assert_eq!(b, 0xbf); }
    #[test] fn set_bit2_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 2, false); assert_eq!(b, 0xdf); }
    #[test] fn set_bit3_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 3, false); assert_eq!(b, 0xef); }
    #[test] fn set_bit4_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 4, false); assert_eq!(b, 0xf7); }
    #[test] fn set_bit5_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 5, false); assert_eq!(b, 0xfb); }
    #[test] fn set_bit6_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 6, false); assert_eq!(b, 0xfd); }
    #[test] fn set_bit7_low() { let mut b: u8 = 0xFF; MD5Byte::set_bit(&mut b, 7, false); assert_eq!(b, 0xfe); }
}
/// MD5計算に用いる32ビットのワード値。
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MD5Word([u8; 4]);
impl MD5Word {
    fn set_bit(&mut self, index: usize, value: bool) {
        assert!(index < 32);
        MD5Byte::set_bit(&mut self.0[index / 8], index % 8, value)
    }
}
impl Default for MD5Word {
    fn default() -> Self { Self([0; 4]) }
}
impl std::ops::Index<usize> for MD5Word {
    type Output = u8;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD5Word {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}
impl std::ops::Add for MD5Word {
    type Output = Self;
    fn add(self, rhs: Self) -> <Self as std::ops::Add<Self>>::Output { <Self as From<u32>>::from(u32::wrapping_add(<Self as Into<u32>>::into(self), <Self as Into<u32>>::into(rhs))) }
}
impl std::ops::AddAssign for MD5Word {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs }
}
impl std::ops::BitAnd for MD5Word {
    type Output = Self;
    fn bitand(self, rhs: Self) -> <Self as std::ops::BitAnd<Self>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self) & <Self as Into<u32>>::into(rhs)) }
}
impl std::ops::BitOr for MD5Word {
    type Output = Self;
    fn bitor(self, rhs: Self) -> <Self as std::ops::BitOr<Self>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self) | <Self as Into<u32>>::into(rhs)) }
}
impl std::ops::BitXor for MD5Word {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> <Self as std::ops::BitXor<Self>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self) ^ <Self as Into<u32>>::into(rhs)) }
}
impl std::ops::Not for MD5Word {
    type Output = Self;
    fn not(self) -> <Self as std::ops::Not>::Output { <Self as From<u32>>::from(!<Self as Into<u32>>::into(self)) }
}
impl std::ops::Shl<u32> for MD5Word {
    type Output = Self;
    fn shl(self, rhs: u32) -> <Self as std::ops::Shl<u32>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self).rotate_left(rhs)) }
}
impl std::ops::Shr<u32> for MD5Word {
    type Output = Self;
    fn shr(self, rhs: u32) -> <Self as std::ops::Shr<u32>>::Output { <Self as From<u32>>::from(<Self as Into<u32>>::into(self).rotate_right(rhs)) }
}
impl std::ops::BitAndAssign for MD5Word {
    fn bitand_assign(&mut self, rhs: Self) { *self = *self & rhs }
}
impl std::ops::BitOrAssign for MD5Word {
    fn bitor_assign(&mut self, rhs: Self) { *self = *self | rhs }
}
impl std::ops::BitXorAssign for MD5Word {
    fn bitxor_assign(&mut self, rhs: Self) { *self = *self ^ rhs }
}
impl From<u32> for MD5Word {
    fn from(value: u32) -> Self { Self(value.to_le_bytes()) }
}
impl Into<u32> for MD5Word {
    fn into(self) -> u32 { u32::from_le_bytes(self.0) }
}
impl Debug for MD5Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("MD5Word({})", <Self as Into<u32>>::into(*self)))
    }
}
#[cfg(test)]
mod md5word_test {
    use crate::md5::MD5Word;
    #[test] fn from_pattern1() { let w = 0x67452301; assert_eq!(<MD5Word as From<u32>>::from(w), MD5Word([0x01, 0x23, 0x45, 0x67])) }
    #[test] fn from_pattern2() { let w = 0xefcdab89; assert_eq!(<MD5Word as From<u32>>::from(w), MD5Word([0x89, 0xab, 0xcd, 0xef])) }
    #[test] fn from_pattern3() { let w = 0x98badcfe; assert_eq!(<MD5Word as From<u32>>::from(w), MD5Word([0xfe, 0xdc, 0xba, 0x98])) }
    #[test] fn from_pattern4() { let w = 0x10325476; assert_eq!(<MD5Word as From<u32>>::from(w), MD5Word([0x76, 0x54, 0x32, 0x10])) }
    #[test] fn into_pattern1() { let w = MD5Word([0x01, 0x23, 0x45, 0x67]); assert_eq!(<MD5Word as Into<u32>>::into(w), 0x67452301) }
    #[test] fn into_pattern2() { let w = MD5Word([0x89, 0xab, 0xcd, 0xef]); assert_eq!(<MD5Word as Into<u32>>::into(w), 0xefcdab89) }
    #[test] fn into_pattern3() { let w = MD5Word([0xfe, 0xdc, 0xba, 0x98]); assert_eq!(<MD5Word as Into<u32>>::into(w), 0x98badcfe) }
    #[test] fn into_pattern4() { let w = MD5Word([0x76, 0x54, 0x32, 0x10]); assert_eq!(<MD5Word as Into<u32>>::into(w), 0x10325476) }
    #[test] fn set_bit0_high() { let mut b = MD5Word::default(); b.set_bit(0, true); assert_eq!(b, MD5Word([0x80, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit1_high() { let mut b = MD5Word::default(); b.set_bit(1, true); assert_eq!(b, MD5Word([0x40, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit2_high() { let mut b = MD5Word::default(); b.set_bit(2, true); assert_eq!(b, MD5Word([0x20, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit3_high() { let mut b = MD5Word::default(); b.set_bit(3, true); assert_eq!(b, MD5Word([0x10, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit4_high() { let mut b = MD5Word::default(); b.set_bit(4, true); assert_eq!(b, MD5Word([0x08, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit5_high() { let mut b = MD5Word::default(); b.set_bit(5, true); assert_eq!(b, MD5Word([0x04, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit6_high() { let mut b = MD5Word::default(); b.set_bit(6, true); assert_eq!(b, MD5Word([0x02, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit7_high() { let mut b = MD5Word::default(); b.set_bit(7, true); assert_eq!(b, MD5Word([0x01, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit8_high() { let mut b = MD5Word::default(); b.set_bit(8, true); assert_eq!(b, MD5Word([0x00, 0x80, 0x00, 0x00])); }
    #[test] fn set_bit9_high() { let mut b = MD5Word::default(); b.set_bit(9, true); assert_eq!(b, MD5Word([0x00, 0x40, 0x00, 0x00])); }
    #[test] fn set_bit10_high() { let mut b = MD5Word::default(); b.set_bit(10, true); assert_eq!(b, MD5Word([0x00, 0x20, 0x00, 0x00])); }
    #[test] fn set_bit11_high() { let mut b = MD5Word::default(); b.set_bit(11, true); assert_eq!(b, MD5Word([0x00, 0x10, 0x00, 0x00])); }
    #[test] fn set_bit12_high() { let mut b = MD5Word::default(); b.set_bit(12, true); assert_eq!(b, MD5Word([0x00, 0x08, 0x00, 0x00])); }
    #[test] fn set_bit13_high() { let mut b = MD5Word::default(); b.set_bit(13, true); assert_eq!(b, MD5Word([0x00, 0x04, 0x00, 0x00])); }
    #[test] fn set_bit14_high() { let mut b = MD5Word::default(); b.set_bit(14, true); assert_eq!(b, MD5Word([0x00, 0x02, 0x00, 0x00])); }
    #[test] fn set_bit15_high() { let mut b = MD5Word::default(); b.set_bit(15, true); assert_eq!(b, MD5Word([0x00, 0x01, 0x00, 0x00])); }
    #[test] fn set_bit16_high() { let mut b = MD5Word::default(); b.set_bit(16, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x80, 0x00])); }
    #[test] fn set_bit17_high() { let mut b = MD5Word::default(); b.set_bit(17, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x40, 0x00])); }
    #[test] fn set_bit18_high() { let mut b = MD5Word::default(); b.set_bit(18, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x20, 0x00])); }
    #[test] fn set_bit19_high() { let mut b = MD5Word::default(); b.set_bit(19, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x10, 0x00])); }
    #[test] fn set_bit20_high() { let mut b = MD5Word::default(); b.set_bit(20, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x08, 0x00])); }
    #[test] fn set_bit21_high() { let mut b = MD5Word::default(); b.set_bit(21, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x04, 0x00])); }
    #[test] fn set_bit22_high() { let mut b = MD5Word::default(); b.set_bit(22, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x02, 0x00])); }
    #[test] fn set_bit23_high() { let mut b = MD5Word::default(); b.set_bit(23, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x01, 0x00])); }
    #[test] fn set_bit24_high() { let mut b = MD5Word::default(); b.set_bit(24, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x80])); }
    #[test] fn set_bit25_high() { let mut b = MD5Word::default(); b.set_bit(25, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x40])); }
    #[test] fn set_bit26_high() { let mut b = MD5Word::default(); b.set_bit(26, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x20])); }
    #[test] fn set_bit27_high() { let mut b = MD5Word::default(); b.set_bit(27, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x10])); }
    #[test] fn set_bit28_high() { let mut b = MD5Word::default(); b.set_bit(28, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x08])); }
    #[test] fn set_bit29_high() { let mut b = MD5Word::default(); b.set_bit(29, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x04])); }
    #[test] fn set_bit30_high() { let mut b = MD5Word::default(); b.set_bit(30, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x02])); }
    #[test] fn set_bit31_high() { let mut b = MD5Word::default(); b.set_bit(31, true); assert_eq!(b, MD5Word([0x00, 0x00, 0x00, 0x01])); }
    #[test] fn set_bit0_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(0, false); assert_eq!(b, MD5Word([0x7f, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit1_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(1, false); assert_eq!(b, MD5Word([0xbf, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit2_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(2, false); assert_eq!(b, MD5Word([0xdf, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit3_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(3, false); assert_eq!(b, MD5Word([0xef, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit4_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(4, false); assert_eq!(b, MD5Word([0xf7, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit5_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(5, false); assert_eq!(b, MD5Word([0xfb, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit6_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(6, false); assert_eq!(b, MD5Word([0xfd, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit7_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(7, false); assert_eq!(b, MD5Word([0xfe, 0xff, 0xff, 0xff])); }
    #[test] fn set_bit8_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(8, false); assert_eq!(b, MD5Word([0xff, 0x7f, 0xff, 0xff])); }
    #[test] fn set_bit9_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(9, false); assert_eq!(b, MD5Word([0xff, 0xbf, 0xff, 0xff])); }
    #[test] fn set_bit10_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(10, false); assert_eq!(b, MD5Word([0xff, 0xdf, 0xff, 0xff])); }
    #[test] fn set_bit11_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(11, false); assert_eq!(b, MD5Word([0xff, 0xef, 0xff, 0xff])); }
    #[test] fn set_bit12_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(12, false); assert_eq!(b, MD5Word([0xff, 0xf7, 0xff, 0xff])); }
    #[test] fn set_bit13_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(13, false); assert_eq!(b, MD5Word([0xff, 0xfb, 0xff, 0xff])); }
    #[test] fn set_bit14_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(14, false); assert_eq!(b, MD5Word([0xff, 0xfd, 0xff, 0xff])); }
    #[test] fn set_bit15_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(15, false); assert_eq!(b, MD5Word([0xff, 0xfe, 0xff, 0xff])); }
    #[test] fn set_bit16_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(16, false); assert_eq!(b, MD5Word([0xff, 0xff, 0x7f, 0xff])); }
    #[test] fn set_bit17_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(17, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xbf, 0xff])); }
    #[test] fn set_bit18_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(18, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xdf, 0xff])); }
    #[test] fn set_bit19_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(19, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xef, 0xff])); }
    #[test] fn set_bit20_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(20, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xf7, 0xff])); }
    #[test] fn set_bit21_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(21, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xfb, 0xff])); }
    #[test] fn set_bit22_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(22, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xfd, 0xff])); }
    #[test] fn set_bit23_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(23, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xfe, 0xff])); }
    #[test] fn set_bit24_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(24, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0x7f])); }
    #[test] fn set_bit25_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(25, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xbf])); }
    #[test] fn set_bit26_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(26, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xdf])); }
    #[test] fn set_bit27_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(27, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xef])); }
    #[test] fn set_bit28_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(28, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xf7])); }
    #[test] fn set_bit29_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(29, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xfb])); }
    #[test] fn set_bit30_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(30, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xfd])); }
    #[test] fn set_bit31_low() { let mut b = MD5Word::from(0xFFFFFFFF); b.set_bit(31, false); assert_eq!(b, MD5Word([0xff, 0xff, 0xff, 0xfe])); }
    #[test] fn and_pattern1() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) & MD5Word([0xfe, 0xdc, 0xba, 0x98]), MD5Word([0x00, 0x00, 0x00, 0x00])) }
    #[test] fn and_pattern2() { assert_eq!(MD5Word([0x89, 0xab, 0xcd, 0xef]) & MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0x00, 0x00, 0x00, 0x00])) }
    #[test] fn and_pattern3() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) & MD5Word([0x89, 0xab, 0xcd, 0xef]), MD5Word([0x01, 0x23, 0x45, 0x67])) }
    #[test] fn and_pattern4() { assert_eq!(MD5Word([0xfe, 0xdc, 0xba, 0x98]) & MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0x76, 0x54, 0x32, 0x10])) }
    #[test] fn or_pattern1() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) | MD5Word([0xfe, 0xdc, 0xba, 0x98]), MD5Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn or_pattern2() { assert_eq!(MD5Word([0x89, 0xab, 0xcd, 0xef]) | MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn or_pattern3() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) | MD5Word([0x89, 0xab, 0xcd, 0xef]), MD5Word([0x89, 0xab, 0xcd, 0xef])) }
    #[test] fn or_pattern4() { assert_eq!(MD5Word([0xfe, 0xdc, 0xba, 0x98]) | MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0xfe, 0xdc, 0xba, 0x98])) }
    #[test] fn xor_pattern1() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) ^ MD5Word([0xfe, 0xdc, 0xba, 0x98]), MD5Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn xor_pattern2() { assert_eq!(MD5Word([0x89, 0xab, 0xcd, 0xef]) ^ MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn xor_pattern3() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) ^ MD5Word([0x89, 0xab, 0xcd, 0xef]), MD5Word([0x88, 0x88, 0x88, 0x88])) }
    #[test] fn xor_pattern4() { assert_eq!(MD5Word([0xfe, 0xdc, 0xba, 0x98]) ^ MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0x88, 0x88, 0x88, 0x88])) }
    #[test] fn not_pattern1() { assert_eq!(!MD5Word([0x01, 0x23, 0x45, 0x67]), MD5Word([0xfe, 0xdc, 0xba, 0x98])) }
    #[test] fn not_pattern2() { assert_eq!(!MD5Word([0x89, 0xab, 0xcd, 0xef]), MD5Word([0x76, 0x54, 0x32, 0x10])) }
    #[test] fn not_pattern3() { assert_eq!(!MD5Word([0xfe, 0xdc, 0xba, 0x98]), MD5Word([0x01, 0x23, 0x45, 0x67])) }
    #[test] fn not_pattern4() { assert_eq!(!MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0x89, 0xab, 0xcd, 0xef])) }
    #[test] fn add_pattern1() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) + MD5Word([0xfe, 0xdc, 0xba, 0x98]), MD5Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn add_pattern2() { assert_eq!(MD5Word([0x89, 0xab, 0xcd, 0xef]) + MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0xff, 0xff, 0xff, 0xff])) }
    #[test] fn add_pattern3() { assert_eq!(MD5Word([0x01, 0x23, 0x45, 0x67]) + MD5Word([0x89, 0xab, 0xcd, 0xef]), MD5Word([0x8a, 0xce, 0x12, 0x57])) }
    #[test] fn add_pattern4() { assert_eq!(MD5Word([0xfe, 0xdc, 0xba, 0x98]) + MD5Word([0x76, 0x54, 0x32, 0x10]), MD5Word([0x74, 0x31, 0xed, 0xa8])) }
}
/// MD5ハッシュ計算時のデータチャンク。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MD5Chunk([MD5Word; 16]);
impl MD5Chunk {
    fn set_bit(&mut self, index: usize, value: bool) {
        assert!(index < 512);
        self[index / 32].set_bit(index % 32, value)
    }
}
impl Default for MD5Chunk {
    fn default() -> Self { Self([MD5Word::default(); 16]) }
}
impl std::ops::Index<usize> for MD5Chunk {
    type Output = MD5Word;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD5Chunk {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}
#[cfg(test)]
mod md5chunk_test {
    use crate::md5::{MD5Word, MD5Chunk};
    #[test] fn set_bit0_high() { let mut b = MD5Chunk::default(); b.set_bit(0, true); assert_eq!(b[0], MD5Word([0x80, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit16_high() { let mut b = MD5Chunk::default(); b.set_bit(16, true); assert_eq!(b[0], MD5Word([0x00, 0x00, 0x80, 0x00])); }
    #[test] fn set_bit33_high() { let mut b = MD5Chunk::default(); b.set_bit(33, true); assert_eq!(b[1], MD5Word([0x40, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit49_high() { let mut b = MD5Chunk::default(); b.set_bit(49, true); assert_eq!(b[1], MD5Word([0x00, 0x00, 0x40, 0x00])); }
    #[test] fn set_bit66_high() { let mut b = MD5Chunk::default(); b.set_bit(66, true); assert_eq!(b[2], MD5Word([0x20, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit82_high() { let mut b = MD5Chunk::default(); b.set_bit(82, true); assert_eq!(b[2], MD5Word([0x00, 0x00, 0x20, 0x00])); }
    #[test] fn set_bit99_high() { let mut b = MD5Chunk::default(); b.set_bit(99, true); assert_eq!(b[3], MD5Word([0x10, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit115_high() { let mut b = MD5Chunk::default(); b.set_bit(115, true); assert_eq!(b[3], MD5Word([0x00, 0x00, 0x10, 0x00])); }
    #[test] fn set_bit132_high() { let mut b = MD5Chunk::default(); b.set_bit(132, true); assert_eq!(b[4], MD5Word([0x08, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit148_high() { let mut b = MD5Chunk::default(); b.set_bit(148, true); assert_eq!(b[4], MD5Word([0x00, 0x00, 0x08, 0x00])); }
    #[test] fn set_bit165_high() { let mut b = MD5Chunk::default(); b.set_bit(165, true); assert_eq!(b[5], MD5Word([0x04, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit181_high() { let mut b = MD5Chunk::default(); b.set_bit(181, true); assert_eq!(b[5], MD5Word([0x00, 0x00, 0x04, 0x00])); }
    #[test] fn set_bit198_high() { let mut b = MD5Chunk::default(); b.set_bit(198, true); assert_eq!(b[6], MD5Word([0x02, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit214_high() { let mut b = MD5Chunk::default(); b.set_bit(214, true); assert_eq!(b[6], MD5Word([0x00, 0x00, 0x02, 0x00])); }
    #[test] fn set_bit231_high() { let mut b = MD5Chunk::default(); b.set_bit(231, true); assert_eq!(b[7], MD5Word([0x01, 0x00, 0x00, 0x00])); }
    #[test] fn set_bit247_high() { let mut b = MD5Chunk::default(); b.set_bit(247, true); assert_eq!(b[7], MD5Word([0x00, 0x00, 0x01, 0x00])); }
    #[test] fn set_bit264_high() { let mut b = MD5Chunk::default(); b.set_bit(264, true); assert_eq!(b[8], MD5Word([0x00, 0x80, 0x00, 0x00])); }
    #[test] fn set_bit280_high() { let mut b = MD5Chunk::default(); b.set_bit(280, true); assert_eq!(b[8], MD5Word([0x00, 0x00, 0x00, 0x80])); }
    #[test] fn set_bit297_high() { let mut b = MD5Chunk::default(); b.set_bit(297, true); assert_eq!(b[9], MD5Word([0x00, 0x40, 0x00, 0x00])); }
    #[test] fn set_bit313_high() { let mut b = MD5Chunk::default(); b.set_bit(313, true); assert_eq!(b[9], MD5Word([0x00, 0x00, 0x00, 0x40])); }
    #[test] fn set_bit330_high() { let mut b = MD5Chunk::default(); b.set_bit(330, true); assert_eq!(b[10], MD5Word([0x00, 0x20, 0x00, 0x00])); }
    #[test] fn set_bit346_high() { let mut b = MD5Chunk::default(); b.set_bit(346, true); assert_eq!(b[10], MD5Word([0x00, 0x00, 0x00, 0x20])); }
    #[test] fn set_bit363_high() { let mut b = MD5Chunk::default(); b.set_bit(363, true); assert_eq!(b[11], MD5Word([0x00, 0x10, 0x00, 0x00])); }
    #[test] fn set_bit379_high() { let mut b = MD5Chunk::default(); b.set_bit(379, true); assert_eq!(b[11], MD5Word([0x00, 0x00, 0x00, 0x10])); }
    #[test] fn set_bit396_high() { let mut b = MD5Chunk::default(); b.set_bit(396, true); assert_eq!(b[12], MD5Word([0x00, 0x08, 0x00, 0x00])); }
    #[test] fn set_bit412_high() { let mut b = MD5Chunk::default(); b.set_bit(412, true); assert_eq!(b[12], MD5Word([0x00, 0x00, 0x00, 0x08])); }
    #[test] fn set_bit429_high() { let mut b = MD5Chunk::default(); b.set_bit(429, true); assert_eq!(b[13], MD5Word([0x00, 0x04, 0x00, 0x00])); }
    #[test] fn set_bit445_high() { let mut b = MD5Chunk::default(); b.set_bit(445, true); assert_eq!(b[13], MD5Word([0x00, 0x00, 0x00, 0x04])); }
    #[test] fn set_bit462_high() { let mut b = MD5Chunk::default(); b.set_bit(462, true); assert_eq!(b[14], MD5Word([0x00, 0x02, 0x00, 0x00])); }
    #[test] fn set_bit478_high() { let mut b = MD5Chunk::default(); b.set_bit(478, true); assert_eq!(b[14], MD5Word([0x00, 0x00, 0x00, 0x02])); }
    #[test] fn set_bit495_high() { let mut b = MD5Chunk::default(); b.set_bit(495, true); assert_eq!(b[15], MD5Word([0x00, 0x01, 0x00, 0x00])); }
    #[test] fn set_bit511_high() { let mut b = MD5Chunk::default(); b.set_bit(511, true); assert_eq!(b[15], MD5Word([0x00, 0x00, 0x00, 0x01])); }
}
/// MD5の計算バッファ。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MD5Buffer([MD5Word; 4]);
impl MD5Buffer {
    fn at_u8(&self, index: usize) -> u8 { self[index/4][index%4] }
}
impl Default for MD5Buffer {
    fn default() -> Self { MD5Buffer([MD5Word([0x01, 0x23, 0x45, 0x67]), MD5Word([0x89, 0xab, 0xcd, 0xef]), MD5Word([0xfe, 0xdc, 0xba, 0x98]), MD5Word([0x76, 0x54, 0x32, 0x10])]) }
}
impl std::ops::Index<usize> for MD5Buffer {
    type Output = MD5Word;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output { &self.0[index] }
}
impl std::ops::IndexMut<usize> for MD5Buffer {
    fn index_mut(&mut self, index: usize) -> &mut <Self as std::ops::Index<usize>>::Output { &mut self.0[index] }
}
impl std::convert::Into<MD5Digest> for MD5Buffer {
    fn into(self) -> MD5Digest {
        let mut result: MD5Digest = [0; 16];
        for i in 0..16 {
            result[i] = self.at_u8(i);
        }
        result
    }
}
#[cfg(test)]
mod md5buffer_test {
    use crate::md5::{MD5Buffer, MD5Digest};
    #[test] fn convert() { assert_eq!(<MD5Buffer as Into<MD5Digest>>::into(MD5Buffer::default()), [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]); }
}
