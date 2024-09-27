#![allow(dead_code)]

pub const DIGIT_TO_BASE10_SQUARED: [u8; 200] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
    b'0', b'8', b'0', b'9', b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5',
    b'1', b'6', b'1', b'7', b'1', b'8', b'1', b'9', b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3',
    b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7', b'2', b'8', b'2', b'9', b'3', b'0', b'3', b'1',
    b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7', b'3', b'8', b'3', b'9',
    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
    b'4', b'8', b'4', b'9', b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5',
    b'5', b'6', b'5', b'7', b'5', b'8', b'5', b'9', b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3',
    b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7', b'6', b'8', b'6', b'9', b'7', b'0', b'7', b'1',
    b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7', b'7', b'8', b'7', b'9',
    b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8', b'5', b'8', b'6', b'8', b'7',
    b'8', b'8', b'8', b'9', b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4', b'9', b'5',
    b'9', b'6', b'9', b'7', b'9', b'8', b'9', b'9',
];

#[inline(always)]
pub const fn fast_log2(x: u32) -> usize {
    32 - 1 - (x | 1).leading_zeros() as usize
}

#[inline(always)]
pub const fn fast_log10(x: u32) -> usize {
    let log2 = fast_log2(x);
    (log2 * 1233) >> 12
}

#[inline(always)]
pub const fn fast_digit_count(x: u32) -> usize {
    const TABLE: [u64; 32] = [
        4294967296,
        8589934582,
        8589934582,
        8589934582,
        12884901788,
        12884901788,
        12884901788,
        17179868184,
        17179868184,
        17179868184,
        21474826480,
        21474826480,
        21474826480,
        21474826480,
        25769703776,
        25769703776,
        25769703776,
        30063771072,
        30063771072,
        30063771072,
        34349738368,
        34349738368,
        34349738368,
        34349738368,
        38554705664,
        38554705664,
        38554705664,
        41949672960,
        41949672960,
        41949672960,
        42949672960,
        42949672960,
    ];
    let shift = TABLE[fast_log2(x)];
    let count = (x as u64 + shift) >> 32;
    count as usize
}

pub const fn digit_count(x: u32) -> usize {
    fast_digit_count(x)
}

#[inline(always)]
pub fn copy_to_dst<T: Copy, Bytes: AsRef<[T]>>(dst: &mut [T], src: Bytes) -> usize {
    let src = src.as_ref();
    dst[..src.len()].copy_from_slice(src);

    src.len()
}

#[inline(always)]
pub const fn digit_to_char_const(digit: u32, radix: u32) -> u8 {
    if radix <= 10 || digit < 10 {
        // Can short-circuit if we know the radix is small at compile time.
        digit as u8 + b'0'
    } else {
        digit as u8 + b'A' - 10
    }
}

macro_rules! i {
    ($array:ident[$index:expr]) => {
        unsafe { $array.get_unchecked($index) }
    };
}

#[macro_export]
macro_rules! write_digit {
    ($buffer:ident, $index:expr, $digit:expr, $checked:expr) => {{
        $index -= 1;
        if $checked {
            $buffer[$index] = $digit;
        } else {
            unsafe {
                let ptr = $buffer.get_unchecked_mut($index) as *mut u8;
                core::ptr::write(ptr, $digit);
            }
        }
    }};
}

#[macro_export]
macro_rules! write_digits {
    ($buffer:ident, $index:expr, $r:expr, $table:ident, $checked:expr) => {{
        write_digit!($buffer, $index, *i!($table[$r + 1]), $checked);
        write_digit!($buffer, $index, *i!($table[$r]), $checked);
    }};
}
