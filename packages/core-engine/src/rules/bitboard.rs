use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Deref
};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitboard(pub u64);

// ==============================================================================
// 1. Deref for Native Bit-Twiddling Access
// ==============================================================================
impl Deref for Bitboard {
    type Target = u64;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ==============================================================================
// 2. Core Convenience Methods
// ==============================================================================
impl Bitboard {
    pub const EMPTY: Self = Bitboard(0);
    pub const ALL: Self = Bitboard(!0);

    /// Create a new bitboard wrapper.
    #[inline(always)]
    pub const fn new(val: u64) -> Self {
        Bitboard(val)
    }

    /// Check if the bitboard is completely empty.
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Check if a specific bit index (0..63) is populated.
    #[inline(always)]
    pub fn has_bit(self, index: u8) -> bool {
        ((self.0 >> index) & 1) != 0
    }

    /// Isolates the least significant 1-bit (LSB). Stockfish equivalent: `bb & -bb`
    #[inline(always)]
    pub fn least_significant_bit(self) -> Self {
        Bitboard(self.0 & (!self.0 + 1))
    }

    /// Pops the lowest bit off the board and returns its index. 
    /// Essential for fast move generation loops.
    #[inline(always)]
    pub fn pop_lsb(&mut self) -> u8 {
        let index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1; // Clears the lowest set bit
        index
    }
}

// ==============================================================================
// 3. Debug Formatting (Beautiful 8x8 Grid Layout for Terminal Debugging)
// ==============================================================================
impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bitboard(0x{:016X}):", self.0)?;
        for rank in (0..8).rev() {
            write!(f, " {} |", rank + 1)?;
            for file in 0..8 {
                let idx = rank * 8 + file;
                if self.has_bit(idx) {
                    write!(f, " 1")?;
                } else {
                    write!(f, " .")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "     -----------------")?;
        writeln!(f, "       A B C D E F G H")
    }
}

// ==============================================================================
// 4. Overloading Operator Traits (No more .0 required for operations!)
// ==============================================================================

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline(always)] fn bitand(self, rhs: Self) -> Self { Bitboard(self.0 & rhs.0) }
}
impl BitOr for Bitboard {
    type Output = Self;
    #[inline(always)] fn bitor(self, rhs: Self) -> Self { Bitboard(self.0 | rhs.0) }
}
impl BitXor for Bitboard {
    type Output = Self;
    #[inline(always)] fn bitxor(self, rhs: Self) -> Self { Bitboard(self.0 ^ rhs.0) }
}
impl Not for Bitboard {
    type Output = Self;
    #[inline(always)] fn not(self) -> Self { Bitboard(!self.0) }
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    #[inline(always)] fn shl(self, rhs: u8) -> Self { Bitboard(self.0 << rhs) }
}
impl Shr<u8> for Bitboard {
    type Output = Self;
    #[inline(always)] fn shr(self, rhs: u8) -> Self { Bitboard(self.0 >> rhs) }
}

impl BitAndAssign for Bitboard { #[inline(always)] fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; } }
impl BitOrAssign for Bitboard { #[inline(always)] fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; } }
impl BitXorAssign for Bitboard { #[inline(always)] fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; } }
impl ShlAssign<u8> for Bitboard { #[inline(always)] fn shl_assign(&mut self, rhs: u8) { self.0 <<= rhs; } }
impl ShrAssign<u8> for Bitboard { #[inline(always)] fn shr_assign(&mut self, rhs: u8) { self.0 >>= rhs; } }