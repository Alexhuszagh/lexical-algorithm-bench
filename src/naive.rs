use crate::shared::{copy_to_dst, digit_to_char_const, DigitCount};

macro_rules! naive {
    ($name:ident, $t:ty) => {
        #[inline(always)]
        unsafe fn $name<const CHECKED: bool>(mut value: $t, buffer: &mut [u8]) -> usize {
            let mut index = buffer.len();
            while value >= 10 {
                let r = value % 10;
                value /= 10;
                let digit = digit_to_char_const(r as u32, 10);
                write_digit!(buffer, index, digit, CHECKED);
            }

            let r = value % 10;
            let digit = digit_to_char_const(r as u32, 10);
            write_digit!(buffer, index, digit, CHECKED);

            index
        }
    };
}

naive!(naive8, u8);
naive!(naive16, u16);
naive!(naive32, u32);
naive!(naive64, u64);
// TODO: Make u128 faster
naive!(naive128, u128);

macro_rules! temp {
    ($name:ident, $t:ty, $meth:ident, $size:literal) => {
        // Version that uses an exact digit count to avoid a temp buffer.
        #[inline(always)]
        pub fn $name<const CHECKED: bool>(value: $t, buffer: &mut [u8]) -> &mut [u8] {
            let mut digits: [u8; $size] = [0u8; $size];
            let index = unsafe { $meth::<CHECKED>(value, &mut digits) };
            let slc = &digits[index..];
            let count = copy_to_dst(buffer, slc);
            &mut buffer[..count]
        }
    };
}

temp!(naive_temp8, u8, naive8, 3);
temp!(naive_temp16, u16, naive16, 5);
temp!(naive_temp32, u32, naive32, 10);
temp!(naive_temp64, u64, naive64, 20);
// TODO: Make u128 faster

macro_rules! exact {
    ($name:ident, $t:ty, $meth:ident) => {
        // Version that uses an exact digit count to avoid a temp buffer.
        #[inline(always)]
        pub fn $name<const CHECKED: bool>(value: $t, buffer: &mut [u8]) -> &mut [u8] {
            let count = value.digit_count();
            let buffer = &mut buffer[..count];
            _ = unsafe { $meth::<CHECKED>(value, buffer) };

            buffer
        }
    };
}

exact!(naive_exact8, u8, naive8);
exact!(naive_exact16, u16, naive16);
exact!(naive_exact32, u32, naive32);
exact!(naive_exact64, u64, naive64);
exact!(naive_exact128, u128, naive128);
