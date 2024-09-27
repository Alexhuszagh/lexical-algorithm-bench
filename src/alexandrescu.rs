use crate::shared::{digit_count, digit_to_char_const, DIGIT_TO_BASE10_SQUARED};

macro_rules! decode4 {
    (@yes $buffer:ident, $index:ident, $value:ident) => {{
        // Decode 4 digits at a time.
        while $value >= 10000 {
            let r = $value % 10000;
            $value /= 10000;
            let r1 = 2 * r / 100;
            let r2 = 2 * (r % 100);

            write_digits!(
                $buffer,
                $index,
                r2 as usize,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            write_digits!(
                $buffer,
                $index,
                r1 as usize,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
        }
    }};

    (@no $buffer:ident, $index:ident, $value:ident) => {};
}

macro_rules! alexandrescu {
    ($name:ident, $t:ty, $decode4:ident) => {
        // Version that uses an exact digit count to avoid a temp buffer.
        #[inline(always)]
        pub fn $name<const CHECKED: bool>(mut value: $t, buffer: &mut [u8]) -> &mut [u8] {
            let count = digit_count(value as _);
            let buffer = &mut buffer[..count];
            let mut index = buffer.len();

            decode4!(@$decode4 buffer, index, value);

            // Decode 2 digits at a time.
            while value >= 100 {
                let r = 2 * value % 100;
                value /= 100;
                write_digits!(buffer, index, r as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
            }

            if value < 10 {
                let digit = digit_to_char_const(value as _, 10);
                // SAFETY: this is always safe, since value < radix, so it must be < 36.
                write_digit!(buffer, index, digit, CHECKED);
            } else {
                let r = 2 * value;
                write_digits!(buffer, index, r as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
            }

            buffer
        }
    };
}

alexandrescu!(alexandrescu8, u8, no);
alexandrescu!(alexandrescu16, u16, yes);
alexandrescu!(alexandrescu32, u32, yes);
// TODO: Restore for the u64 digit count
//alexandrescu!(alexandrescu64, u64);
// TODO: u128 is too slow, make it faster
