use crate::shared::{copy_to_dst, digit_count, digit_to_char_const};

// Version that uses a temporary buffer.
#[inline(always)]
pub fn naive32_temp<const CHECKED: bool>(value: u32, buffer: &mut [u8]) -> usize {
    let mut digits: [u8; 128] = [0u8; 128];
    let index = unsafe { naive32::<CHECKED>(value, &mut digits) };
    let slc = &digits[index..];
    copy_to_dst(buffer, slc)
}

// Version that uses an exact digit count to avoid a temp buffer.
#[inline(always)]
pub fn naive32_exact<const CHECKED: bool>(value: u32, buffer: &mut [u8]) -> &mut [u8] {
    let count = digit_count(value);
    let buffer = &mut buffer[..count];
    _ = unsafe { naive32::<CHECKED>(value, buffer) };

    buffer
}

// Version that uses a temporary buffer
#[inline(always)]
unsafe fn naive32<const CHECKED: bool>(mut value: u32, buffer: &mut [u8]) -> usize {
    let mut index = buffer.len();
    while value >= 10 {
        let r = value % 10;
        value /= 10;
        let digit = digit_to_char_const(r, 10);
        write_digit!(buffer, index, digit, CHECKED);
    }

    let r = value % 10;
    let digit = digit_to_char_const(r, 10);
    write_digit!(buffer, index, digit, CHECKED);

    index
}
