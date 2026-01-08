use std::{
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

pub struct BitMasher<T> {
    frame_size: u8,
    frame_count: u8,
    _marker: PhantomData<T>,
}

impl BitMasher<u128> {
    pub fn new(frame_size: u8, frame_count: u8) -> Self {
        Self {
            frame_size,
            frame_count,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn frame_mask(&self, i: u8) -> u128 {
        self.frame_shift(((1u128 << (self.frame_size + 1)) - 1), i)
    }

    #[inline]
    pub fn frame_shift(&self, frame: u128, i: u8) -> u128 {
        frame << i * self.frame_size
    }

    #[inline]
    pub fn checked_sub(&self, left: &u128, right: &u128) -> Option<u128> {
        for i in 0..self.frame_count {
            let mask = self.frame_mask(i);
            if (*left & mask) < (*right & mask) {
                return None;
            }
        }
        Some(*left - *right)
    }

    #[inline]
    pub fn is_even(&self, data: &u128) -> bool {
        for i in 0..10 {
            if (*data & (self.frame_shift(0b1, i))) > 0 {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn write_frame(&self, data: &u128, frame: u128, i: u8) -> u128 {
        let mask = self.frame_mask(i);
        (*data & !mask) | (self.frame_shift(frame, i) & mask)
    }

    #[inline]
    pub fn read_frame(&self, data: &u128, i: u8) -> u128 {
        (*data & self.frame_mask(i)) >> i * self.frame_size
    }
}
