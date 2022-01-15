#[cfg(test)] mod keccakword64_test;
#[cfg(test)] mod keccakstatearray_test;

use std::convert::TryFrom;

/// Keccakでビット、またはバイト単位のアクセスをサポートするためのゲッター。
#[derive(Clone, Copy)]
pub enum KeccakDataGetter<T> {
    Byte(fn (&T, usize) -> u8, usize),
    Bit(fn (&T, usize) -> bool, usize)
}
impl<T> KeccakDataGetter<T> {
    pub fn len(&self) -> usize { match self { Self::Bit(_, l) => *l, Self::Byte(_, l) => *l } }
    pub fn as_byte(&self) -> Option<(fn (&T, usize) -> u8, usize)> { if let Self::Byte(f, l) = self { Some((*f, *l)) } else { None } }
    pub fn as_bit(&self) -> Option<(fn (&T, usize) -> bool, usize)> { if let Self::Bit(f, l) = self { Some((*f, *l)) } else { None } }
    pub fn is_byte(&self) -> bool { if let Self::Byte(_, _) = self { true } else { false } }
    pub fn is_bit(&self) -> bool { if let Self::Bit(_, _) = self { true } else { false } }
}

/// Keccakでビット、またはバイト単位のアクセスをサポートするためのセッター。
#[derive(Clone, Copy)]
pub enum KeccakDataSetter<T> {
    Byte(fn (&mut T, usize, u8), usize),
    Bit(fn (&mut T, usize, bool), usize)
}
impl<T> KeccakDataSetter<T> {
    pub fn len(&self) -> usize { match self { Self::Bit(_, l) => *l, Self::Byte(_, l) => *l } }
    pub fn as_byte(&self) -> Option<(fn (&mut T, usize, u8), usize)> { if let Self::Byte(f, l) = self { Some((*f, *l)) } else { None } }
    pub fn as_bit(&self) -> Option<(fn (&mut T, usize, bool), usize)> { if let Self::Bit(f, l) = self { Some((*f, *l)) } else { None } }
    pub fn is_byte(&self) -> bool { if let Self::Byte(_, _) = self { true } else { false } }
    pub fn is_bit(&self) -> bool { if let Self::Bit(_, _) = self { true } else { false } }
}

/// Keccakで用いられるビット集合を表すための実装を表します。
/// この実装はKeccakのstate array内でlaneとして用いられます。
pub trait KeccakWord<T = Self> :
    Default +
    Copy +
    Eq +
    std::ops::Not<Output = T> +
    std::ops::BitAnd<T, Output = T> +
    std::ops::BitXor<T, Output = T> +
    std::ops::Shl<u32, Output = T> +
    std::ops::ShlAssign<u32> +
    std::ops::BitXorAssign<T>
{
    /// このワードのビット幅。
    const W: usize;
    /// このワードのビット幅の2を底とする対数。
    const L: usize;
    /// このワードに効率的にアクセスするためのゲッター。
    const GETTER: KeccakDataGetter<T>;
    /// このワードに効率的にアクセスするためのセッター。
    const SETTER: KeccakDataSetter<T>;
    /// 指定された位置のビットを取得します。
    fn get_bit(&self, index: usize) -> bool;
    /// 指定された位置のビットを設定します。
    fn set_bit(&mut self, index: usize, value: bool);
}

/// `KeccakSponge`への入力を表します。
pub trait KeccakSpongeInputArray<T = Self> : Default {
    /// Keccak spongeとの入出力のレート。
    const R: usize;
    /// このオブジェクトからデータを取得するためのゲッター。
    const GETTER: KeccakDataGetter<T>;
    /// このオブジェクトへデータを設定するためのセッター。
    const SETTER: KeccakDataSetter<T>;
    /// このオブジェクトの指定されたビットを取得します。
    fn get_bit(&self, index: usize) -> bool;
    /// このオブジェクトの指定されたビットを設定します。
    fn set_bit(&mut self, index: usize, value: bool);
}

/// Keccakで使用される64ビットワード。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeccakWord64(u64);
impl KeccakWord64 {
    fn get_byte(&self, index: usize) -> u8 { assert!(index < 8); u8::try_from((self.0 >> (index * 8)) & 0xFF).unwrap() }
    fn set_byte(&mut self, index: usize, value: u8) { assert!(index < 8); self.0 = (self.0 & !(0xFF << (index * 8))) | (u64::from(value) << (index * 8)) }
}
impl KeccakWord for KeccakWord64 {
    const W: usize = 64;
    const L: usize = 6;
    const GETTER: KeccakDataGetter<Self> = KeccakDataGetter::Byte(Self::get_byte, 8);
    const SETTER: KeccakDataSetter<Self> = KeccakDataSetter::Byte(Self::set_byte, 8);
    fn get_bit(&self, index: usize) -> bool { assert!(index < 64); ((self.0 >> index) & 1) != 0 }
    fn set_bit(&mut self, index: usize, value: bool) { assert!(index < 64); self.0 = (self.0 & !(1 << index)) | (if value { 1 } else { 0 } << index) }
}
impl Default for KeccakWord64 {
    fn default() -> Self { KeccakWord64(0) }
}
impl std::ops::Not for KeccakWord64 {
    type Output = Self;
    fn not(self) -> <Self as std::ops::Not>::Output { Self(!self.0) }
}
impl std::ops::BitAnd for KeccakWord64 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> <Self as std::ops::BitAnd<Self>>::Output { Self(self.0 & rhs.0) }
}
impl std::ops::BitAndAssign for KeccakWord64 {
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0 }
}
impl std::ops::BitXor for KeccakWord64 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> <Self as std::ops::BitXor<Self>>::Output { Self(self.0 ^ rhs.0) }
}
impl std::ops::BitXorAssign for KeccakWord64 {
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0 }
}
impl std::ops::Shl<u32> for KeccakWord64 {
    type Output = Self;
    fn shl(self, rhs: u32) -> <Self as std::ops::Shl<u32>>::Output { Self(self.0.rotate_left(rhs)) }
}
impl std::ops::ShlAssign<u32> for KeccakWord64 {
    fn shl_assign(&mut self, rhs: u32) { *self = *self << rhs }
}
impl std::convert::Into<[u8; 8]> for KeccakWord64 {
    fn into(self) -> [u8; 8] { self.0.to_le_bytes() }
}
impl std::convert::From<[u8; 8]> for KeccakWord64 {
    fn from(value: [u8; 8]) -> Self { Self(u64::from_le_bytes(value)) }
}

/// Keccakで用いられるstate arrayを表します。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeccakStateArray<T: KeccakWord>([T; 25]);
impl<T: KeccakWord> KeccakStateArray<T> {
    pub const B: usize = T::W * 25;
    /// このstate arrayの指定したlaneを取得します。
    pub fn lane(&self, x: usize, y: usize) -> &T { assert!(x < 5); assert!(y < 5); &self.0[y * 5 + x] }
    /// このstate arrayの指定した変更可能なlaneを取得します。
    pub fn lane_mut(&mut self, x: usize, y: usize) -> &mut T { assert!(x < 5); assert!(y < 5); &mut self.0[y * 5 + x] }
    /// このstate arrayの指定したビットを取得します。
    pub fn get_array_bit(&self, x: usize, y: usize, z: usize) -> bool { self.lane(x, y).get_bit(z) }
    /// このstate arrayの指定したビットを設定します。
    pub fn set_array_bit(&mut self, x: usize, y: usize, z: usize, value: bool) { self.lane_mut(x, y).set_bit(z, value) }
    /// このstate arrayに効率的にアクセスするためのゲッターを取得します。
    pub fn getter() -> KeccakDataGetter<Self> {
        match T::GETTER {
            KeccakDataGetter::Byte(_, l) => KeccakDataGetter::Byte(Self::get_byte, 25 * l),
            KeccakDataGetter::Bit(_, l) => KeccakDataGetter::Bit(Self::get_string_bit, 25 * l)
        }
    }
    /// このstate arrayに効率的にアクセスするためのセッターを取得します。
    pub fn setter() -> KeccakDataSetter<Self> {
        match T::SETTER {
            KeccakDataSetter::Byte(_, l) => KeccakDataSetter::Byte(Self::set_byte, 25 * l),
            KeccakDataSetter::Bit(_, l) => KeccakDataSetter::Bit(Self::set_string_bit, 25 * l)
        }
    }
    /// このstate arrayの指定したビットを取得します。
    fn get_byte(&self, index: usize) -> u8 {
        let g = T::GETTER.as_byte().unwrap();
        assert!(index < (25 * g.1));
        g.0(&self.0[index / g.1], index % g.1)
    }
    /// このstate arrayの指定したビットを設定します。
    fn set_byte(&mut self, index: usize, value: u8) {
        let s = T::SETTER.as_byte().unwrap();
        assert!(index < (25 * s.1));
        s.0(&mut self.0[index / s.1], index % s.1, value)
    }
    /// このstate arrayをstringとしたときの指定したビットを取得します。
    pub fn get_string_bit(&self, index: usize) -> bool {
        let z = index % T::W;
        self.0[index / T::W].get_bit(z)
    }
    /// このstate arrayをstringとしたときの指定したビットを設定します。
    pub fn set_string_bit(&mut self, index: usize, value: bool) {
        let z = index % T::W;
        self.0[index / T::W].set_bit(z, value)
    }
    /// このstate arrayに対し、Keccak-fを行います。
    pub fn keccak_f(&mut self) {
        let iteration = 12 + 2 * T::L;
        self.keccak_p(iteration);
    }
    /// このstate arrayに対し、Keccak-pを行います。
    pub fn keccak_p(&mut self, rounds: usize) {
        let rmax = 12 + 2 * isize::try_from(T::L).unwrap();
        let rmin = rmax - isize::try_from(rounds).unwrap();
        for i in rmin..rmax {
            self.op_theta();
            self.op_rho();
            self.op_pi();
            self.op_chi();
            self.op_iota(i);
        }
    }
    fn op_theta(&mut self) {
        let mut c = [T::default(); 5];
        for x in 0..5 { for y in 0..5 { c[x] ^= *self.lane(x, y); } }
        let c = c;
        let mut d = [T::default(); 5];
        for x in 0..5 { d[x] = c[(x + 4) % 5] ^ (c[(x + 1) % 5] << 1) }
        let d = d;
        for x in 0..5 { for y in 0..5 { *self.lane_mut(x, y) ^= d[x]; } }
    }
    fn op_rho(&mut self) {
        const ROTATE_TABLE: [u32; 25] = [
              0,   1, 190,  28,  91,
             36, 300,   6,  55, 276,
              3,  10, 171, 153, 231,
            105,  45,  15,  21, 136,
            210,  66, 253, 120,  78
        ];
        for y in 0..5 { for x in 0..5 { *self.lane_mut(x, y) <<= ROTATE_TABLE[5 * y + x] % u32::try_from(T::W).unwrap() } }
    }
    fn swap_item(&mut self, l: (usize, usize), r: (usize, usize)) {
            let t = *self.lane(l.0, l.1);
            *self.lane_mut(l.0, l.1) = *self.lane(r.0, r.1);
            *self.lane_mut(r.0, r.1) = t;
    }
    fn op_pi(&mut self) {
        let mut p: (usize, usize) = (1, 1);
        let pursue = |p: (usize, usize)| ((p.0 + p.1 * 3) % 5, p.0);
        for _ in 0..23 {
            let dest = p;
            p = pursue(p);
            self.swap_item(p, dest);
        }
    }
    fn op_chi(&mut self) {
        for y in 0..5 {
            let mut c = [T::default(); 5];
            for x in 0..5 { c[x] = (!*self.lane((x + 1) % 5, y)) & *self.lane((x + 2) % 5, y); }
            for x in 0..5 { *self.lane_mut(x, y) ^= c[x]; }
        }
    }
    fn op_rc(t: isize) -> bool {
        let n = t % 255;
        if n == 0 { return true }
        let mut r: u8 = 1;
        for _ in 0..n {
            let b = (r & (1 << 7)) != 0;
            r = ((r << 1) & !1) ^ if b { 0x71 } else { 0x00 };
        }
        (r & 1) != 0
    }
    fn op_iota(&mut self, ri: isize) {
        let mut rc = T::default();
        for j in 0..(T::L + 1) { rc.set_bit((1 << j) - 1, Self::op_rc(7 * ri + isize::try_from(j).unwrap())); }
        *self.lane_mut(0, 0) ^= rc
    }
}
impl<T: KeccakWord> Default for KeccakStateArray<T> {
    fn default() -> Self { Self([T::default(); 25]) }
}

/// Keccakで用いられるsponge構造を表します。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeccakSponge<T: KeccakWord>(KeccakStateArray<T>);
impl<T: KeccakWord> KeccakSponge<T> {
    /// spongeにデータを「吸収」します。
    pub fn absorb<I: KeccakSpongeInputArray>(&mut self, in_data: &I) {
        assert!(I::R <= KeccakStateArray::<T>::B);
        let dest_g = KeccakStateArray::<T>::getter();
        let dest_s = KeccakStateArray::<T>::setter();
        if I::GETTER.is_byte() && dest_s.is_byte() && dest_g.is_byte() {
            let src = I::GETTER.as_byte().unwrap();
            let dest_s = dest_s.as_byte().unwrap();
            let dest_g = dest_g.as_byte().unwrap();
            assert_eq!(dest_g.1, dest_s.1);
            assert!(src.1 <= dest_s.1);
            assert!((src.1 * 8) == I::R);
            for i in 0..src.1 {
                let v = dest_g.0(&self.0, i) ^ src.0(in_data, i);
                dest_s.0(&mut self.0, i, v);
            }
        } else {
            for i in 0..I::R {
                let v = self.0.get_string_bit(i) ^ in_data.get_bit(i);
                self.0.set_string_bit(i, v);
            }
        }
        self.0.keccak_f();
    }
    /// spongeからデータを「絞り出」します。
    pub fn squeeze<O: KeccakSpongeInputArray>(&mut self) -> O {
        assert!(O::R <= KeccakStateArray::<T>::B);
        let mut result = O::default();
        let src = KeccakStateArray::<T>::getter();
        if O::SETTER.is_byte() && src.is_byte() {
            let src = src.as_byte().unwrap();
            let dest = O::SETTER.as_byte().unwrap();
            assert!(dest.1 <= src.1);
            for i in 0..dest.1 { dest.0(&mut result, i, src.0(&self.0, i)) }
        }
        else {
            for i in 0..O::R { result.set_bit(i, self.0.get_string_bit(i)) }
        }
        self.0.keccak_f();
        result
    }
}
impl<T: KeccakWord> Default for KeccakSponge<T> {
    fn default() -> Self { Self(KeccakStateArray::<T>::default()) }
}
