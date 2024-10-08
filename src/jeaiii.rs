#![allow(dead_code, unused)]

use crate::alexandrescu::alexandrescu64;
use crate::shared::{copy_to_dst, digit_to_char_const, DigitCount, DIGIT_TO_BASE10_SQUARED};

macro_rules! write_digit_i {
    ($buffer:ident, $index:ident, $digit:expr, $checked:expr) => {{
        let digit = $digit;
        if $checked {
            $buffer[$index] = digit;
        } else {
            unsafe {
                let ptr = $buffer.get_unchecked_mut($index) as *mut u8;
                core::ptr::write(ptr, digit);
            }
        }
        $index += 1;
    }};
}

macro_rules! write_digits_i {
    ($buffer:ident, $index:ident, $r:expr, $table:ident, $checked:expr) => {{
        let r = $r as usize;
        write_digit_i!($buffer, $index, *i!($table[r]), $checked);
        write_digit_i!($buffer, $index, *i!($table[r + 1]), $checked);
    }};
}

// Optimized version when printing exactly 10 digits.
// This contains leading 0s.
// NOTE: This does **NOT** work for values outside the range, that is, between 99_9999_9998
// and 99_9999_9999, so it is only safe up to u32_max.
#[inline(always)]
pub fn jeaiii32_10<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    let buffer = &mut buffer[..10];
    let mut index = 0;
    const SHIFT: i32 = 57;
    const MASK: u64 = (1u64 << SHIFT) - 1;
    const LO: u64 = u32::MAX as u64;

    let mut y = (n as u64) * 1441151881;
    write_digits_i!(buffer, index, (y >> SHIFT) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
    y = (y & MASK) * 100;
    write_digits_i!(buffer, index, (y >> SHIFT) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
    y = (y & MASK) * 100;
    write_digits_i!(buffer, index, (y >> SHIFT) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
    y = (y & MASK) * 100;
    write_digits_i!(buffer, index, (y >> SHIFT) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
    y = (y & MASK) * 100;
    write_digits_i!(buffer, index, (y >> SHIFT) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);

    buffer
}

// Optimized version when printing exactly 10 digits.
// This contains no leading 0s.
// Performance is terrible, as expected, due to the memcpy.
#[inline(always)]
pub fn jeaiii32_10_start<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    let count = n.digit_count();
    assert!(buffer.len() >= 10);
    let buffer = jeaiii32_10::<CHECKED>(n, buffer);
    unsafe {
        let src = buffer.as_ptr().add(10 - count);
        let dst = buffer.as_mut_ptr();
        core::ptr::copy(src, dst, count);
    }
    &mut buffer[..count]
}

macro_rules! print_1 {
    ($buffer:ident, $index:ident, $y:expr, $checked:expr) => {{
        let y = $y;
        let digit = digit_to_char_const((y >> 32) as u32, 10);
        write_digit_i!($buffer, $index, digit, $checked);
    }};
}

// Original versiom of the jeaiii algorithm
#[inline(always)]
pub fn jeaiii32_original<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    let buffer = &mut buffer[..10];
    let mut index = 0;
    const LO: u64 = u32::MAX as u64;

    if n < 100 {
        if n < 10 {
            let digit = digit_to_char_const(n, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            write_digits_i!(buffer, index, n as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..2]
        }
    } else if n < 100_0000 {
        if n < 1_0000 {
            // 3 or 4 digits.
            // 42949673 = ceil(2^32 / 10^2)
            let mut y = n as u64 * 42949673u64;
            if n < 1000 {
                print_1!(buffer, index, y, CHECKED);
                y = (y & LO) * 100;
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                &mut buffer[..3]
            } else {
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                y = (y & LO) * 100;
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                &mut buffer[..4]
            }
        } else {
            // 5 or 6 digits.
            // 429497 = ceil(2^32 / 10^4)
            let mut y = n as u64 * 429497u64;
            if n < 10_0000 {
                // 5 digits.
                let digit = digit_to_char_const((y >> 32) as u32, 10);
                write_digit_i!(buffer, index, digit, CHECKED);
                y = (y & LO) * 100;
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                y = (y & LO) * 100;
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                &mut buffer[..5]
            } else {
                // 6 digits.
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                y = (y & LO) * 100;
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                y = (y & LO) * 100;
                write_digits_i!(
                    buffer,
                    index,
                    (y >> 32) as u32 * 2,
                    DIGIT_TO_BASE10_SQUARED,
                    CHECKED
                );
                &mut buffer[..6]
            }
        }
    } else if n < 1_0000_0000 {
        // 7 or 8 digits.
        // 140737489 = ceil(2^47 / 10^6)
        let mut y = n as u64 * 140737489u64;
        let mask = (1u64 << 47) - 1;
        if n < 1000_0000 {
            // 7 digits.
            let digit = digit_to_char_const((y >> 47) as u32, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..7]
        } else {
            // 8 digits.
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 47) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..8]
        }
    } else {
        // 9 or 10 digits.
        // 1441151881 = ceil(2^57 / 10^8)
        let mut y = n as u64 * 1441151881u64;
        let mask = (1u64 << 57) - 1;
        if n < 10_0000_0000 {
            // 9 digits.
            let digit = digit_to_char_const((y >> 57) as u32, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..9]
        } else {
            // 10 digits.
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            y = (y & mask) * 100;
            write_digits_i!(buffer, index, (y >> 57) as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..10]
        }
    }
}

#[inline(always)]
fn next2(prod: &mut u64) -> u32 {
    const LO: u64 = u32::MAX as u64;
    *prod = (*prod & LO) * 100;
    (*prod >> 32) as u32
}

macro_rules! print_2 {
    ($buffer:ident, $index:ident, $prod:ident, $checked:ident) => {
        write_digits_i!($buffer, $index, next2(&mut $prod) * 2, DIGIT_TO_BASE10_SQUARED, $checked);
    };
}

macro_rules! print_i {
    (
        $buffer:ident,
        $index:ident,
        $checked:ident,
        $n:ident,
        $magic:expr,
        $shift:expr,
        $remaining:expr
    ) => {{
        let mut prod = ($n as u64) * $magic;
        prod >>= $shift;
        let two = (prod >> 32) as u32;
        if two < 10 {
            let digit = digit_to_char_const(two, 10);
            write_digit_i!($buffer, $index, digit, $checked);
            for _ in 0..$remaining {
                print_2!($buffer, $index, prod, $checked);
            }
        } else {
            write_digits_i!($buffer, $index, two as u32 * 2, DIGIT_TO_BASE10_SQUARED, $checked);
            for _ in 0..$remaining {
                print_2!($buffer, $index, prod, $checked);
            }
        }
    }};
}

// Better versiom of the jeaiii algorithm
// NOTE: This one is not correct for 9-10 digits it seems
// Not a big deal since the performance difference is minimal between this and
// the original
#[inline(always)]
pub fn jeaiii32_better<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    let buffer = &mut buffer[..10];
    let mut index = 0;

    if n < 100 {
        if n < 10 {
            let digit = digit_to_char_const(n, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            write_digits_i!(buffer, index, n as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..2]
        }
    } else {
        if n < 100_0000 {
            if n < 1_0000 {
                // 3 or 4 digits.
                // 42949673 = ceil(2^32 / 10^2)
                print_i!(buffer, index, CHECKED, n, 42949673u64, 0, 1);
                &mut buffer[..index]
            } else {
                // 5 or 6 digits.
                // 429497 = ceil(2^32 / 10^4)
                print_i!(buffer, index, CHECKED, n, 429497u64, 0, 2);
                &mut buffer[..index]
            }
        } else {
            if n < 1_0000_0000 {
                // 7 or 8 digits.
                // 281474978 = ceil(2^48 / 10^6) + 1
                print_i!(buffer, index, CHECKED, n, 281474978u64, 16, 3);
                &mut buffer[..index]
            } else {
                if n < 10_0000_0000 {
                    // 9 digits.
                    // 1441151882 = ceil(2^57 / 10^8) + 1
                    let mut prod = (n as u64) * 1441151882u64;
                    prod >>= 25;
                    let digit = digit_to_char_const((prod >> 32) as u32, 10);
                    write_digit_i!(buffer, index, digit, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    &mut buffer[..index]
                } else {
                    // 10 digits.
                    // 1441151881 = ceil(2^57 / 10^8)
                    let mut prod = (n as u64) * 1441151881u64;
                    prod >>= 25;
                    write_digits_i!(
                        buffer,
                        index,
                        (prod >> 32) as u32 * 2,
                        DIGIT_TO_BASE10_SQUARED,
                        CHECKED
                    );
                    print_2!(buffer, index, prod, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    print_2!(buffer, index, prod, CHECKED);
                    &mut buffer[..index]
                }
            }
        }
    }
}

// This pre-calculates the number of digits.
// This is significantly slower and therefore should not be used.
// The digit count calculation seems to be very slow and then the compiler
// can't optimize all the checks because of the initial buffer index.
#[inline(always)]
pub fn jeaiii32_digits<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    let count = n.digit_count();
    let buffer = &mut buffer[..count];
    let mut index = 0;

    match count {
        1 => {
            let digit = digit_to_char_const(n, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
        },
        2 => write_digits_i!(buffer, index, n as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED),
        3 | 4 => print_i!(buffer, index, CHECKED, n, 42949673u64, 0, 1),
        5 | 6 => print_i!(buffer, index, CHECKED, n, 429497u64, 0, 2),
        7 | 8 => print_i!(buffer, index, CHECKED, n, 281474978u64, 16, 3),
        9 => {
            // 9 digits.
            // 1441151882 = ceil(2^57 / 10^8) + 1
            let mut prod = (n as u64) * 1441151882u64;
            prod >>= 25;
            let digit = digit_to_char_const((prod >> 32) as u32, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
        },
        _ => {
            // 10 digits.
            // 1441151881 = ceil(2^57 / 10^8)
            let mut prod = (n as u64) * 1441151881u64;
            prod >>= 25;
            write_digits_i!(
                buffer,
                index,
                (prod >> 32) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
        },
    }

    buffer
}

// Better versiom of the jeaiii algorithm
// NOTE: This one is not correct for 9-10 digits it seems
// Not a big deal since the performance difference is minimal between this and
// the original
#[inline(always)]
pub fn jeaiii8_better<const CHECKED: bool>(n: u8, buffer: &mut [u8]) -> &mut [u8] {
    let buffer = &mut buffer[..10];
    let mut index = 0;

    if n < 10 {
        let digit = digit_to_char_const(n as _, 10);
        write_digit_i!(buffer, index, digit, CHECKED);
        &mut buffer[..1]
    } else if n < 100 {
        write_digits_i!(buffer, index, n as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
        &mut buffer[..2]
    } else {
        // 3 or 4 digits.
        // 42949673 = ceil(2^32 / 10^2)
        print_i!(buffer, index, CHECKED, n, 42949673u64, 0, 1);
        &mut buffer[..index]
    }
}

// Hack attempt to implement a u8 version in terms of a 32-bit one
#[inline(always)]
pub fn jeaiii8as32_better<const CHECKED: bool>(n: u8, buffer: &mut [u8]) -> &mut [u8] {
    jeaiii32_better::<CHECKED>(n as u32, buffer)
}

// Better versiom of the jeaiii algorithm
// NOTE: This one is not correct for 9-10 digits it seems
// Not a big deal since the performance difference is minimal between this and
// the original
#[inline(always)]
pub fn jeaiii16_better<const CHECKED: bool>(n: u16, buffer: &mut [u8]) -> &mut [u8] {
    let buffer = &mut buffer[..10];
    let mut index = 0;

    if n < 100 {
        if n < 10 {
            let digit = digit_to_char_const(n as _, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            write_digits_i!(buffer, index, n as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
            &mut buffer[..2]
        }
    } else {
        if n < 1_0000 {
            // 3 or 4 digits.
            // 42949673 = ceil(2^32 / 10^2)
            print_i!(buffer, index, CHECKED, n, 42949673u64, 0, 1);
            &mut buffer[..index]
        } else {
            // 5 or 6 digits.
            // 429497 = ceil(2^32 / 10^4)
            print_i!(buffer, index, CHECKED, n, 429497u64, 0, 2);
            &mut buffer[..index]
        }
    }
}

// Hack attempt to implement a u16 version in terms of a 32-bit one
#[inline(always)]
pub fn jeaiii16as32_better<const CHECKED: bool>(n: u16, buffer: &mut [u8]) -> &mut [u8] {
    jeaiii32_better::<CHECKED>(n as u32, buffer)
}

// Basic implementation using standard division to write a 64-bit integer to
// bytes. This uses standard division to get the upper and/or lower bytes from
// it.
#[inline(always)]
pub fn jeaiii64_better<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    const FACTOR: u64 = 10_0000_0000;
    if n <= U32_MAX {
        // Up to 10 digits
        jeaiii32_better::<CHECKED>(n as u32, buffer)
    } else if n <= U32_MAX * FACTOR {
        // NOTE: Our lo will be 9 digits, so we can write those digits
        // 2nd and then re-assign the higher bits, which makes this very
        // easy.

        // 11-19 digits
        let hi = (n / FACTOR) as u32;
        let lo = (n % FACTOR) as u32;
        // NOTE: We store this value so we can write 10, then just re-assign.
        // We just always get the first 20 so the bounds checks can be elided.
        let buffer = &mut buffer[..20];
        let count = jeaiii32_better::<CHECKED>(hi, buffer).len();
        let index = count.saturating_sub(1);
        let last = buffer[index];
        let buffer = &mut buffer[..index + 10];
        _ = jeaiii32_10::<CHECKED>(lo, &mut buffer[index..]);
        buffer[index] = last;
        buffer
    } else {
        // have 19-20 digits
        let buffer = &mut buffer[..20];
        // hi can be 1-2 digits, the rest are 9 digits
        let mid = n / FACTOR;
        let hi = (mid / FACTOR) as u32;
        let mid = (mid % FACTOR) as u32;
        let lo = (n % FACTOR) as u32;
        // NOTE: We need to write this after
        let index: usize = if hi < 10 {
            0
        } else {
            1
        };
        // NOTE: This depends on index, it's 0 or 1
        _ = jeaiii32_10::<CHECKED>(lo, &mut buffer[index + 9..]);
        _ = jeaiii32_10::<CHECKED>(mid, &mut buffer[index..index + 10]);
        if hi < 10 {
            buffer[0] = digit_to_char_const(hi as u32, 10);
        } else {
            let r = 2 * hi as usize;
            buffer[0] = *i!(DIGIT_TO_BASE10_SQUARED[r]);
            buffer[1] = *i!(DIGIT_TO_BASE10_SQUARED[r + 1]);
        }

        &mut buffer[0..index + 19]
    }
}

// Basic implementation using standard division to write a 64-bit integer to
// bytes. This uses standard division to get the upper and/or lower bytes from
// it.
// NOTE: This seems to have pretty bad performance and is only slightly better than v1
#[inline(always)]
pub fn jeaiii64_better_v2<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    const FACTOR: u64 = 10_0000_0000;
    if n <= U32_MAX {
        // Up to 10 digits
        jeaiii32_better::<CHECKED>(n as u32, buffer)
    } else {
        // 10-20 digits
        // Do an index check here so we can elide all later checks
        let buffer = &mut buffer[..20];
        let mut div = n / FACTOR;
        let lo = (n % FACTOR) as u32;

        let mut index: usize = 0;
        if div <= U32_MAX {
            // Going to need to test where we have
            // have 10-19 digits, have `hi` (1-9 digits) and `lo` (10 digits)
            // we can guarantee `hi` is in 9 digits because `u32_max / FACTOR`
            // is at most 9 digits.
            index = jeaiii32_better::<CHECKED>(div as u32, buffer).len() - 1;
        } else {
            // have 19-20 digits, have `hi` (1-2 digits), `mid` (9 digits), and `lo` (9
            // digits)
            let hi = (div / FACTOR) as u32;
            div = div % FACTOR;
            if hi < 10 {
                buffer[0] = digit_to_char_const(hi as u32, 10);
                index = 0;
            } else {
                let r = 2 * hi as usize;
                buffer[0] = *i!(DIGIT_TO_BASE10_SQUARED[r]);
                buffer[1] = *i!(DIGIT_TO_BASE10_SQUARED[r + 1]);
                index = 1;
            }

            // store the last index since we'll override it with our subsequent write
            let last = buffer[index];
            _ = jeaiii32_10::<CHECKED>(div as u32, &mut buffer[index..]);
            buffer[index] = last;
            index += 9;
        }

        // same thing, just with the `lo` digits
        //index -= 1;
        let last = buffer[index];
        _ = jeaiii32_10::<CHECKED>(lo, &mut buffer[index..]);
        buffer[index] = last;

        &mut buffer[..index + 10]
    }
}

// Basic implementation using standard division to write a 64-bit integer to
// bytes. This uses standard division to get the upper and/or lower bytes from
// it.
// NOTE: This has terrible performance and should not be used.
#[inline(always)]
pub fn jeaiii64_better_v3<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    const FACTOR: u64 = 10_0000_0000;
    if n <= U32_MAX {
        // Up to 10 digits
        jeaiii32_better::<CHECKED>(n as u32, buffer)
    } else {
        // 10-20 digits
        // Do an index check here so we can elide all later checks
        let buffer = &mut buffer[..20];
        let div = n / FACTOR;
        let lo = (n % FACTOR) as u32;
        let index = alexandrescu64::<false>(div, buffer).len() - 1;
        let last = buffer[index];
        _ = jeaiii32_10::<CHECKED>(lo, &mut buffer[index..]);
        buffer[index] = last;
        &mut buffer[..index + 10]
    }
}

// This checks to see if the value can be done in 32-bits or can be done with
// a small factor to break into an easy hi/lo words. This falls back in other
// cases to the Alexandrescu algorithm.
#[inline(always)]
pub fn jeaiii64_better_v4<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    const FACTOR: u64 = 10_0000_0000;
    if n <= U32_MAX {
        // Up to 10 digits
        jeaiii32_better::<CHECKED>(n as u32, buffer)
    } else if n <= U32_MAX * FACTOR {
        // NOTE: Our lo will be 9 digits, so we can write those digits
        // 2nd and then re-assign the higher bits, which makes this very
        // easy.

        // 11-19 digits
        let hi = (n / FACTOR) as u32;
        let lo = (n % FACTOR) as u32;
        // NOTE: We store this value so we can write 10, then just re-assign.
        // We just always get the first 20 so the bounds checks can be elided.
        let buffer = &mut buffer[..20];
        let count = jeaiii32_better::<CHECKED>(hi, buffer).len();
        let index = count.saturating_sub(1);
        let last = buffer[index];
        let buffer = &mut buffer[..index + 10];
        _ = jeaiii32_10::<CHECKED>(lo, &mut buffer[index..]);
        buffer[index] = last;
        buffer
    } else {
        // just do our naive, 2-digit algorithm to avoid any performance issues
        // due to the minimal branching this seems to have way higher performance
        // than our overly-branched implementations
        alexandrescu64::<false>(n, buffer)
    }
}

// Very simple approach with just the faster, jeaiii algorithm for 32-bits and
// the Alexandrescu for the 64-bits one.
// NOTE: This is slow **EXCEPT** for the safe_int benchmarks which seems
// benchmarks which seem to be faster for this (same with large safe_int)
#[inline(always)]
pub fn jeaiii64_better_v5<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    if n <= U32_MAX {
        jeaiii32_better::<CHECKED>(n as u32, buffer)
    } else {
        alexandrescu64::<false>(n, buffer)
    }
}

// This attempts a fully-flattened version
// NOTE: This really isn't fast...
#[inline(always)]
pub fn jeaiii64_better_v6<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    let mut index = 0;
    let mut buffer = &mut buffer[..20];

    if n < 100_0000 {
        if n < 10 {
            let digit = digit_to_char_const(n as u32, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            if n < 100 {
                write_digits_i!(buffer, index, n as u32 * 2, DIGIT_TO_BASE10_SQUARED, CHECKED);
                &mut buffer[..2]
            } else {
                if n < 1_0000 {
                    // 3 or 4 digits.
                    // 42949673 = ceil(2^32 / 10^2)
                    print_i!(buffer, index, CHECKED, n, 42949673u64, 0, 1);
                    &mut buffer[..index]
                } else {
                    // 5 or 6 digits.
                    // 429497 = ceil(2^32 / 10^4)
                    print_i!(buffer, index, CHECKED, n, 429497u64, 0, 2);
                    &mut buffer[..index]
                }
            }
        }
    } else if n < U32_MAX * 2 {
        if n < 1_0000_0000 {
            // 7 or 8 digits.
            // 281474978 = ceil(2^48 / 10^6) + 1
            print_i!(buffer, index, CHECKED, n, 281474978u64, 16, 3);
            &mut buffer[..index]
        } else if n < 10_0000_0000 {
            // 9 digits.
            // 1441151882 = ceil(2^57 / 10^8) + 1
            let mut prod = (n as u64) * 1441151882u64;
            prod >>= 25;
            let digit = digit_to_char_const((prod >> 32) as u32, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            &mut buffer[..index]
        } else {
            // 10 digits.
            // 1441151881 = ceil(2^57 / 10^8)
            let mut prod = (n as u64) * 1441151881u64;
            prod >>= 25;
            write_digits_i!(
                buffer,
                index,
                (prod >> 32) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            print_2!(buffer, index, prod, CHECKED);
            &mut buffer[..index]
        }
    } else {
        alexandrescu64::<false>(n, buffer)
    }
}
