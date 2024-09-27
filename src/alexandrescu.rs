
use crate::shared::{DIGIT_TO_BASE10_SQUARED, digit_count, digit_to_char_const};

// Version that uses an exact digit count to avoid a temp buffer.
#[inline(always)]
pub fn alexandrescu32<const CHECKED: bool>(mut value: u32, buffer: &mut [u8]) -> &mut [u8] {
    let count = digit_count(value);
    let buffer = &mut buffer[..count];
    let mut index = buffer.len();

    // Decode 4 digits at a time.
    while value >= 10000 {
        let r = value % 10000;
        value /= 10000;
        let r1 = 2 * r / 100;
        let r2 = 2 * (r % 100);

        write_digits!(buffer, index, r2 as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
        write_digits!(buffer, index, r1 as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
    }

    // Decode 2 digits at a time.
    while value >= 100 {
        let r = 2 * value % 100;
        value /= 100;
        write_digits!(buffer, index, r as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
    }

    if value < 10 {
        let digit = digit_to_char_const(value, 10);
        // SAFETY: this is always safe, since value < radix, so it must be < 36.
        write_digit!(buffer, index, digit, CHECKED);
    } else {
        let r = 2 * value;
        write_digits!(buffer, index, r as usize, DIGIT_TO_BASE10_SQUARED, CHECKED);
    }

    buffer
}
