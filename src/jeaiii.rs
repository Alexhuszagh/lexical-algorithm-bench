#![allow(dead_code, unused)]

use crate::shared::{digit_count, digit_to_char_const, DIGIT_TO_BASE10_SQUARED};

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
#[inline(always)]
pub fn jeaiii32_10<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    //let count = digit_count(n);
    let buffer = &mut buffer[..10];
    let mut index = 0;
    const SHIFT: i32 = 57;
    const MASK: u64 = (1u64 << SHIFT) - 1;
    const LO: u64 = u32::MAX as u64;

    let mut y = (n as u64) * 1441151881;
    write_digits_i!(
        buffer,
        index,
        (y >> SHIFT) as u32 * 2,
        DIGIT_TO_BASE10_SQUARED,
        CHECKED
    );
    y = (y & MASK) * 100;
    write_digits_i!(
        buffer,
        index,
        (y >> SHIFT) as u32 * 2,
        DIGIT_TO_BASE10_SQUARED,
        CHECKED
    );
    y = (y & MASK) * 100;
    write_digits_i!(
        buffer,
        index,
        (y >> SHIFT) as u32 * 2,
        DIGIT_TO_BASE10_SQUARED,
        CHECKED
    );
    y = (y & MASK) * 100;
    write_digits_i!(
        buffer,
        index,
        (y >> SHIFT) as u32 * 2,
        DIGIT_TO_BASE10_SQUARED,
        CHECKED
    );
    y = (y & MASK) * 100;
    write_digits_i!(
        buffer,
        index,
        (y >> SHIFT) as u32 * 2,
        DIGIT_TO_BASE10_SQUARED,
        CHECKED
    );

    buffer
}

// Optimized version when printing exactly 10 digits.
// This contains no leading 0s.
// Performance is terrible, as expected, due to the memcpy.
#[inline(always)]
pub fn jeaiii32_10_start<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    let count = digit_count(n);
    assert!(count <= 10);
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
    //let count = digit_count(n);
    let buffer = &mut buffer[..10];
    let mut index = 0;
    const LO: u64 = u32::MAX as u64;

    if n < 100 {
        if n < 10 {
            let digit = digit_to_char_const(n, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            write_digits_i!(
                buffer,
                index,
                n as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
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
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            &mut buffer[..7]
        } else {
            // 8 digits.
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 47) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
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
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            &mut buffer[..9]
        } else {
            // 10 digits.
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
            y = (y & mask) * 100;
            write_digits_i!(
                buffer,
                index,
                (y >> 57) as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
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
        write_digits_i!(
            $buffer,
            $index,
            next2(&mut $prod) * 2,
            DIGIT_TO_BASE10_SQUARED,
            $checked
        );
    };
}

macro_rules! print_i {
    ($buffer:ident, $index:ident, $checked:ident, $n:ident, $magic:expr, $shift:expr, $remaining:expr) => {{
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
            write_digits_i!(
                $buffer,
                $index,
                two as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                $checked
            );
            for _ in 0..$remaining {
                print_2!($buffer, $index, prod, $checked);
            }
        }
    }};
}

// Better versiom of the jeaiii algorithm
// NOTE: This one is not correct for 9-10 digits it seems
// Not a big deal since the performance difference is minimal between this and the original
#[inline(always)]
pub fn jeaiii32_better<const CHECKED: bool>(n: u32, buffer: &mut [u8]) -> &mut [u8] {
    //let count = digit_count(n);
    let buffer = &mut buffer[..10];
    let mut index = 0;

    if n < 100 {
        if n < 10 {
            let digit = digit_to_char_const(n, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            write_digits_i!(
                buffer,
                index,
                n as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
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
    let count = digit_count(n);
    let buffer = &mut buffer[..count];
    let mut index = 0;

    match count {
        1 => {
            let digit = digit_to_char_const(n, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
        }
        2 => write_digits_i!(
            buffer,
            index,
            n as u32 * 2,
            DIGIT_TO_BASE10_SQUARED,
            CHECKED
        ),
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
        }
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
        }
    }

    buffer
}

// Better versiom of the jeaiii algorithm
// NOTE: This one is not correct for 9-10 digits it seems
// Not a big deal since the performance difference is minimal between this and the original
#[inline(always)]
pub fn jeaiii8_better<const CHECKED: bool>(n: u8, buffer: &mut [u8]) -> &mut [u8] {
    //let count = digit_count(n);
    let buffer = &mut buffer[..10];
    let mut index = 0;

    if n < 10 {
        let digit = digit_to_char_const(n as _, 10);
        write_digit_i!(buffer, index, digit, CHECKED);
        &mut buffer[..1]
    } else if n < 100 {
        write_digits_i!(
            buffer,
            index,
            n as u32 * 2,
            DIGIT_TO_BASE10_SQUARED,
            CHECKED
        );
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
// Not a big deal since the performance difference is minimal between this and the original
#[inline(always)]
pub fn jeaiii16_better<const CHECKED: bool>(n: u16, buffer: &mut [u8]) -> &mut [u8] {
    //let count = digit_count(n);
    let buffer = &mut buffer[..10];
    let mut index = 0;

    if n < 100 {
        if n < 10 {
            let digit = digit_to_char_const(n as _, 10);
            write_digit_i!(buffer, index, digit, CHECKED);
            &mut buffer[..1]
        } else {
            write_digits_i!(
                buffer,
                index,
                n as u32 * 2,
                DIGIT_TO_BASE10_SQUARED,
                CHECKED
            );
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
