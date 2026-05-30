use std::ops::{Deref, DerefMut};

use crate::rules::bitboard::Bitboard;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    payload: u16,
}

const MAX_MOVE_COUNT:usize = 6*16;

pub struct MoveList {
    moves: [Move; MAX_MOVE_COUNT],
    count: usize,
}


impl Move {
    const FROM_MASK: u16 = 0x003F; // Lowest 6 bits
    const TO_MASK: u16   = 0x0FC0; // Middle 6 bits
    const FLAG_MASK: u16 = 0xF000; // Top 4 bits
    pub const CAPTURE_FLAG: u8 = 0x1;
    
    #[inline]
    pub const fn is_capture(&self) -> bool {
        (self.flags() & Self::CAPTURE_FLAG) != 0
    }

    #[inline]
    pub const fn new(from: u8, to: u8, flags: u8) -> Self {
        Self {
            payload: (from as u16 & Self::FROM_MASK)
                | ((to as u16) << 6 & Self::TO_MASK)
                | ((flags as u16) << 12 & Self::FLAG_MASK),
        }
    }

    #[inline]
    pub const fn from_square(&self) -> u8 {
        (self.payload & Self::FROM_MASK) as u8
    }

    #[inline]
    pub const fn to_square(&self) -> u8 {
        ((self.payload & Self::TO_MASK) >> 6) as u8
    }

    #[inline]
    pub const fn flags(&self) -> u8 {
        ((self.payload & Self::FLAG_MASK) >> 12) as u8
    }

    #[inline]
    pub fn extract_moves_from_mask(
        from_idx: u8, 
        mut destination_mask: Bitboard,
        enemy_pieces: Bitboard,
        flags: u8, 
        move_list: &mut MoveList
    ) {
        
        while !destination_mask.is_empty() {
            let to_idx = destination_mask.pop_lsb();
            let to_mask = Bitboard::from_square(to_idx);
            let flags = if !(destination_mask & to_mask).is_empty() || !(enemy_pieces & to_mask).is_empty() {
                Self::CAPTURE_FLAG
            } else {
                0
            };
            move_list.push(Self::new(from_idx, to_idx, flags));
        }
    }
}

impl MoveList {
    /// Creates an empty, stack-allocated move list.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            moves: [Move::new(0, 0, 0); MAX_MOVE_COUNT],
            count: 0,
        }
    }

    /// Pushes a new move into the stack storage if space permits.
    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        if self.count < MAX_MOVE_COUNT {
            self.moves[self.count] = m;
            self.count += 1;
        }
    }

    /// Returns the active number of moves stored in the list.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Checks if the list contains zero valid elements.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns a borrowed slice containing only the valid moves populated so far.
    #[inline(always)]
    pub fn as_slice(&self) -> &[Move] {
        &self.moves[0..self.count]
    }

    /// Returns a mutable slice containing only the valid moves populated so far.
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [Move] {
        &mut self.moves[0..self.count]
    }

    #[inline]
    pub fn move_to_front(&mut self, index: usize) {
        if index == 0 || index >= self.count {
            return;
        }
        // Save the targeted element
        let target = self.moves[index];
        // Shift intermediate elements right by 1 position to clear index 0
        for i in (1..=index).rev() {
            self.moves[i] = self.moves[i - 1];
        }
        self.moves[0] = target;
    }

}

impl Deref for MoveList {
    type Target = [Move];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.moves[0..self.count]
    }
}

impl DerefMut for MoveList {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.moves[0..self.count]
    }
}

// Concrete iterator type to avoid dynamic box allocations
pub struct MoveListIntoIter {
    list: MoveList,
    index: usize,
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = MoveListIntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        MoveListIntoIter {
            list: self,
            index: 0,
        }
    }
}

impl Iterator for MoveListIntoIter {
    type Item = Move;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.count {
            let m = self.list.moves[self.index];
            self.index += 1;
            Some(m)
        } else {
            None
        }
    }
}