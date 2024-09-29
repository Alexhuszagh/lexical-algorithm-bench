use crate::shared::{digit_to_char_const, DigitCount, DIGIT_TO_BASE10_SQUARED};

macro_rules! decode4 {
    (@yes $buffer:ident, $index:ident, $value:ident) => {{
        // Decode 4 digits at a time.
        while $value >= 10000 {
            let r = $value % 10000;
            $value /= 10000;
            // NOTE: These parentheses are required or we can have `2 * 999 / 100`
            // which overruns our buffer.
            let r1 = 2 * (r / 100);
            let r2 = 2 * (r % 100);

            write_digits!($buffer, $index, r2 as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
            write_digits!($buffer, $index, r1 as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
        }
    }};

    (@no $buffer:ident, $index:ident, $value:ident) => {};
}

macro_rules! alexandrescu {
    ($name:ident, $t:ty, $decode4:ident) => {
        // Version that uses an exact digit count to avoid a temp buffer.
        #[inline(always)]
        pub fn $name<const CHECKED: bool>(mut value: $t, buffer: &mut [u8]) -> &mut [u8] {
            let count = value.digit_count();
            let buffer = &mut buffer[..count];
            let mut index = buffer.len();

            decode4!(@$decode4 buffer, index, value);

            // Decode 2 digits at a time.
            while value >= 100 {
                let r = 2 * (value % 100);
                value /= 100;
                write_digits!(buffer, index, r as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
            }

            if value < 10 {
                let digit = digit_to_char_const(value as u32, 10);
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
// TODO: Add in version using `idiv` or the faster u128 division instructions
// later
alexandrescu!(alexandrescu64, u64, yes);
alexandrescu!(alexandrescu128, u128, yes);
