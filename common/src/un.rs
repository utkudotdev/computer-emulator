use crate::bit_array::BitArray;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, BitAnd, BitOr, Not, Shl, Shr, Sub, SubAssign};

#[macro_export]
macro_rules! bytes_to_store_bits {
    ($n:expr) => {
        ($n - 1) / 8 + 1
    };
}

#[derive(Debug, Copy, Clone)]
pub struct U<const N: usize>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    value: BitArray<N>,
}

impl<const N: usize> U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    pub fn new() -> Self {
        U {
            value: BitArray::zeroes(),
        }
    }

    pub fn with_value(value: BitArray<N>) -> Self {
        U { value }
    }

    pub fn max() -> Self {
        U {
            value: BitArray::ones(),
        }
    }

    pub fn min() -> Self {
        U {
            value: BitArray::zeroes(),
        }
    }

    pub fn change_bits<const M: usize>(self) -> U<M>
    where
        [(); bytes_to_store_bits!(M)]: Sized,
    {
        U {
            value: self.value.change_bits(),
        }
    }
}

// Important difference between U<N> and standard types: U<N> over/underflows by default.
impl<const N: usize> Add for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>; // Add with overflow

    fn add(mut self, rhs: Self) -> Self::Output {
        let mut carry = false;
        for i in 0..self.value.len() {
            let n1 = self.value.get(i);
            let n2 = rhs.value.get(i);

            let sum = n1 ^ n2 ^ carry;
            carry = (n1 & n2) | (n1 & carry) | (n2 & carry);
            self.value.set(i, sum)
        }

        U::with_value(self.value)
    }
}

impl<const N: usize> AddAssign for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn add_assign(&mut self, rhs: Self) {
        // TODO: efficiency
        *self = *self + rhs
    }
}

impl<const N: usize> Sub for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (!rhs + 1u8.into())
    }
}

impl<const N: usize> SubAssign for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<const N: usize> Not for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>;

    fn not(self) -> Self::Output {
        U::with_value(self.value.map(|b| !b))
    }
}

impl<const N: usize> BitAnd for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for i in 0..self.value.len() {
            let n1 = self.value.get(i);
            let n2 = rhs.value.get(i);
            self.value.set(i, n1 & n2)
        }

        U::with_value(self.value)
    }
}

impl<const N: usize> BitOr for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        for i in 0..self.value.len() {
            let n1 = self.value.get(i);
            let n2 = rhs.value.get(i);
            self.value.set(i, n1 | n2)
        }

        U::with_value(self.value)
    }
}

impl<const N: usize> Shr<usize> for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>;

    fn shr(mut self, rhs: usize) -> Self::Output {
        for i in 0..self.value.len() {
            let grab_index = i + rhs;
            let new = if grab_index < self.value.len() {
                self.value.get(grab_index)
            } else {
                false
            };
            self.value.set(i, new);
        }
        self
    }
}

impl<const N: usize> Shl<usize> for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    type Output = U<N>;

    fn shl(mut self, rhs: usize) -> Self::Output {
        for i in (0..self.value.len()).rev() {
            if i < rhs {
                self.value.set(i, false);
            } else {
                let grab_index = i - rhs;
                let new = self.value.get(grab_index);
                self.value.set(i, new);
            }
        }
        self
    }
}

impl<const N: usize> Ord for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn cmp(&self, other: &Self) -> Ordering {
        for (n1, n2) in self.value.iter().zip(other.value.iter()).rev() {
            if n1 > n2 {
                return Ordering::Greater;
            } else if n1 < n2 {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl<const N: usize> PartialOrd<Self> for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> PartialEq<Self> for U<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<const N: usize> Eq for U<N> where [(); bytes_to_store_bits!(N)]: Sized {}

macro_rules! impl_from_primitive {
    ($un:ty, $bits:expr) => {
        impl<const N: usize> From<$un> for U<N>
        where
            [(); bytes_to_store_bits!(N)]: Sized,
        {
            fn from(value: $un) -> Self {
                let bytes = <$un>::to_le_bytes(value);
                let ba = BitArray::<$bits>::from_array(bytes);
                U::with_value(ba.change_bits())
            }
        }
    };
}

macro_rules! impl_to_primitive {
    ($un:ty, $bits:expr) => {
        impl<const N: usize> From<U<N>> for $un
        where
            [(); bytes_to_store_bits!(N)]: Sized,
        {
            fn from(value: U<N>) -> Self {
                let bytes = value.value.change_bits::<$bits>().to_array();
                <$un>::from_le_bytes(bytes)
            }
        }
    };
}

// Passing the expression { ux::BITS as usize } fails type checking for little reason, even when
// the proper where bounds are included (to my knowledge). So we have to resort to this
impl_from_primitive!(u8, 8);
impl_from_primitive!(u16, 16);
impl_from_primitive!(u32, 32);
impl_from_primitive!(u64, 62);
impl_from_primitive!(u128, 128);
// Due to the above, this becomes impossible to implement for usize. Hopefully we don't need it.
// impl_from_primitive!(usize);

// Same here
impl_to_primitive!(u8, 8);
impl_to_primitive!(u16, 16);
impl_to_primitive!(u32, 32);
impl_to_primitive!(u64, 64);
impl_to_primitive!(u128, 128);
// impl_to_primitive!(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_primitive() {
        let a: U<128> = u128::MAX.into();
        assert_eq!(a.value, BitArray::from_array([u8::MAX; 128 / 8]));

        let b: U<16> = u32::MAX.into();
        assert_eq!(b.value, BitArray::from_array([u8::MAX, u8::MAX]));

        let c: U<12> = u16::MAX.into();
        assert_eq!(c.value, BitArray::from_array([u8::MAX, 0b00001111]));

        let d: U<4> = 2u8.into();
        assert_eq!(d.value, BitArray::from_array([2]));

        let e: U<9> = 10u8.into();
        assert_eq!(e.value, BitArray::from_array([0b00001010, 0]));

        let f: U<31> = u64::MAX.into();
        assert_eq!(
            f.value,
            BitArray::from_array([u8::MAX, u8::MAX, u8::MAX, 0b01111111])
        )
    }

    #[test]
    fn to_primitive() {
        let a: U<128> = u128::MAX.into();
        assert_eq!(u128::from(a), u128::MAX)
    }

    #[test]
    fn equality() {
        let a: U<4> = 1u8.into();
        let b: U<4> = 1u8.into();
        let c: U<4> = 2u8.into();

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);

        let d: U<128> = u128::MAX.into();
        let e: U<128> = u128::MAX.into();
        let f: U<128> = (u128::MAX - 10).into();

        assert_eq!(d, e);
        assert_ne!(d, f);
        assert_ne!(e, f);
    }

    #[test]
    fn inequality() {
        let a: U<8> = 5u8.into();
        let b: U<8> = 200u8.into();

        assert!(a < b);
        assert!(b > a);

        let c: U<130> = u128::MAX.into();
        let d: U<130> = (u128::MAX - 10).into();

        assert!(c > d);
        assert!(d < c);

        let e: U<10> = 10u8.into();
        let f: U<10> = 10u8.into();

        assert!(!(e > f));
        assert!(!(e < f));
    }

    #[test]
    fn add() {
        let a: U<4> = 8u8.into();
        let b: U<4> = 1u8.into();

        assert_eq!(a + b, 9u8.into());

        let c: U<128> = (9u128 * 10u128.pow(10)).into();
        let d: U<128> = (3u128 * 10u128.pow(10)).into();

        assert_eq!(c + d, (12u128 * 10u128.pow(10)).into());

        let f: U<4> = 0b1111u8.into();
        let g: U<4> = 0b1111u8.into();
        assert_eq!(f + g, 0b1110u8.into());

        let h: U<8> = u8::MAX.into();
        let i: U<8> = u8::MAX.into();
        assert_eq!(h + i, 0b11111110u8.into())
    }

    #[test]
    fn sub() {
        let a: U<4> = 8u8.into();
        let b: U<4> = 1u8.into();

        assert_eq!(a - b, 7u8.into());

        let c: U<128> = (9u128 * 10u128.pow(10)).into();
        let d: U<128> = (3u128 * 10u128.pow(10)).into();

        assert_eq!(c - d, (6u128 * 10u128.pow(10)).into());

        let e: U<9> = 0b000000000u16.into();
        let f: U<9> = 0b000000001u16.into();

        assert_eq!(e - f, 0b111111111u16.into());

        let h: U<15> = 0b1100111_00100001u16.into();
        let i: U<15> = 0b1101111_00100010u16.into();

        assert_eq!(h - i, 0b1110111_11111111u16.into());

        let j: U<16> = 0b01100111_00100001u16.into();
        let k: U<16> = 0b01101111_00100010u16.into();

        assert_eq!(
            u16::from(j - k),
            0b01100111_00100001u16
                .overflowing_sub(0b01101111_00100010u16)
                .0
        )
    }

    #[test]
    fn bit_and() {
        let a: U<4> = 0b1100u8.into();
        let b: U<4> = 0b0101u8.into();

        assert_eq!(a & b, 0b0100u8.into());

        let c: U<13> = 0b1_1000_1111_0111u16.into();
        let d: U<13> = 0b1_1010_0101_1100u16.into();

        assert_eq!(c & d, 0b1_1000_0101_0100u16.into())
    }

    #[test]
    fn bit_or() {
        let a: U<4> = 0b1100u8.into();
        let b: U<4> = 0b0101u8.into();

        assert_eq!(a | b, 0b1101u8.into());

        let c: U<13> = 0b1_1000_1111_0111u16.into();
        let d: U<13> = 0b1_1010_0101_1100u16.into();

        assert_eq!(c | d, 0b1_1010_1111_1111u16.into())
    }

    #[test]
    fn shr() {
        let a: U<8> = 0b01101001u8.into();
        assert_eq!(a >> 5usize, 0b00000011u8.into());

        let b: U<12> = 0b0110_11000111u16.into();
        assert_eq!(b >> 3usize, 0b0000_11011000u16.into());
        assert_eq!(b >> 9usize, 0b0000_00000011u16.into());
    }

    #[test]
    fn shl() {
        let a: U<8> = 0b01101001u8.into();
        assert_eq!(a << 5usize, 0b00100000u8.into());

        let b: U<12> = 0b0110_11000111u16.into();
        assert_eq!(b << 3usize, 0b0110_00111000u16.into());
        assert_eq!(b << 9usize, 0b1110_00000000u16.into());
    }
}
