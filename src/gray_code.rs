#![allow(dead_code)]

pub trait Integer<const N: usize> {
    fn bytes(&self) -> [u8; N];
    fn from_bytes(bytes: [u8; N]) -> Self;
}
impl Integer<1> for u8 {
    fn bytes(&self) -> [u8; size_of::<Self>()] {
        [*self]
    }
    fn from_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
        bytes[0]
    }
}
impl Integer<2> for u16 {
    fn bytes(&self) -> [u8; size_of::<Self>()] {
        self.to_be_bytes()
    }
    fn from_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
        Self::from_be_bytes(bytes)
    }
}
impl Integer<4> for u32 {
    fn bytes(&self) -> [u8; size_of::<Self>()] {
        self.to_be_bytes()
    }
    fn from_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
        Self::from_be_bytes(bytes)
    }
}
impl Integer<8> for u64 {
    fn bytes(&self) -> [u8; size_of::<Self>()] {
        self.to_be_bytes()
    }
    fn from_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
        Self::from_be_bytes(bytes)
    }
}
impl Integer<8> for usize {
    fn bytes(&self) -> [u8; size_of::<Self>()] {
        self.to_be_bytes()
    }
    fn from_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

pub fn binary_to_gray<const N: usize, T: Integer<N>>(n: T) -> T {
    let b = n.bytes();
    let mut g = [0_u8; N];

    g[0] = dbg!(bit(&b, 0) << 7);
    for bit_idx in 1..N * 8 {
        let byte_idx = bit_idx / 8;
        let bt = dbg!(bit(&b, bit_idx - 1)) ^ dbg!(bit(&b, bit_idx));
        let shift = 7 - bit_idx % 8;
        g[byte_idx] |= dbg!(bt) << shift;
    }
    println!("out: 0b{:b}", g[0]);

    T::from_bytes(g)
}

fn bit(bytes: &[u8], bit_idx: usize) -> u8 {
    let byte = bytes[bit_idx / 8];
    let rel_bit_idx = bit_idx % 8;
    (byte >> (7 - rel_bit_idx)) & 1_u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gray_4_bytes() {
        assert_eq!(binary_to_gray(0_u8), 0b0000);
        assert_eq!(binary_to_gray(1_u8), 0b0001);
        assert_eq!(binary_to_gray(2_u8), 0b0011);
        assert_eq!(binary_to_gray(3_u8), 0b0010);
        assert_eq!(binary_to_gray(4_u8), 0b0110);
        assert_eq!(binary_to_gray(5_u8), 0b0111);
        assert_eq!(binary_to_gray(6_u8), 0b0101);
        assert_eq!(binary_to_gray(7_u8), 0b0100);

        assert_eq!(binary_to_gray(8_u8), 0b1100);
        assert_eq!(binary_to_gray(9_u8), 0b1101);
        assert_eq!(binary_to_gray(10_u8), 0b1111);
        assert_eq!(binary_to_gray(11_u8), 0b1110);
        assert_eq!(binary_to_gray(12_u8), 0b1010);
        assert_eq!(binary_to_gray(13_u8), 0b1011);
        assert_eq!(binary_to_gray(14_u8), 0b1001);
        assert_eq!(binary_to_gray(15_u8), 0b1000);
    }
}
