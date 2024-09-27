use proptest::prelude::*;

macro_rules! assert_buff_eq {
    ($left:expr, $right:expr) => {
        let left = $left;
        let right = $right;
        let index = right.len() - left.len();
        assert_eq!(left, &right[index..]);
        assert!(right[..index].iter().all(|&x| x == b'0'));
    };
}

macro_rules! u32_cases {
    ($name:ident, $func:ident, $checked:expr) => {
        #[test]
        fn $name() {
            let mut buffer = [b'\x00'; 16];
            assert_buff_eq!(b"0", algorithms::$func::<$checked>(0u32, &mut buffer));
            assert_buff_eq!(b"1", algorithms::$func::<$checked>(1u32, &mut buffer));
            assert_buff_eq!(b"5", algorithms::$func::<$checked>(5u32, &mut buffer));
            assert_buff_eq!(b"10", algorithms::$func::<$checked>(10u32, &mut buffer));
            assert_buff_eq!(b"11", algorithms::$func::<$checked>(11u32, &mut buffer));
            assert_buff_eq!(b"12", algorithms::$func::<$checked>(12u32, &mut buffer));
            assert_buff_eq!(b"99", algorithms::$func::<$checked>(99u32, &mut buffer));
            assert_buff_eq!(b"100", algorithms::$func::<$checked>(100u32, &mut buffer));
            assert_buff_eq!(b"101", algorithms::$func::<$checked>(101u32, &mut buffer));
            assert_buff_eq!(b"123", algorithms::$func::<$checked>(123u32, &mut buffer));
            assert_buff_eq!(b"999", algorithms::$func::<$checked>(999u32, &mut buffer));
            assert_buff_eq!(b"1000", algorithms::$func::<$checked>(1000u32, &mut buffer));
            assert_buff_eq!(b"1001", algorithms::$func::<$checked>(1001u32, &mut buffer));
            assert_buff_eq!(b"1234", algorithms::$func::<$checked>(1234u32, &mut buffer));
            assert_buff_eq!(b"9999", algorithms::$func::<$checked>(9999u32, &mut buffer));
            assert_buff_eq!(
                b"10000",
                algorithms::$func::<$checked>(10000u32, &mut buffer)
            );
            assert_buff_eq!(
                b"10001",
                algorithms::$func::<$checked>(10001u32, &mut buffer)
            );
            assert_buff_eq!(
                b"12345",
                algorithms::$func::<$checked>(12345u32, &mut buffer)
            );
            assert_buff_eq!(
                b"99999",
                algorithms::$func::<$checked>(99999u32, &mut buffer)
            );
            assert_buff_eq!(
                b"100000",
                algorithms::$func::<$checked>(100000u32, &mut buffer)
            );
            assert_buff_eq!(
                b"100001",
                algorithms::$func::<$checked>(100001u32, &mut buffer)
            );
            assert_buff_eq!(
                b"123456",
                algorithms::$func::<$checked>(123456u32, &mut buffer)
            );
            assert_buff_eq!(
                b"999999",
                algorithms::$func::<$checked>(999999u32, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000",
                algorithms::$func::<$checked>(1000000u32, &mut buffer)
            );
            assert_buff_eq!(
                b"1000001",
                algorithms::$func::<$checked>(1000001u32, &mut buffer)
            );
            assert_buff_eq!(
                b"1234567",
                algorithms::$func::<$checked>(1234567u32, &mut buffer)
            );
            assert_buff_eq!(
                b"9999999",
                algorithms::$func::<$checked>(9999999u32, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000",
                algorithms::$func::<$checked>(10000000u32, &mut buffer)
            );
            assert_buff_eq!(
                b"10000001",
                algorithms::$func::<$checked>(10000001u32, &mut buffer)
            );
            assert_buff_eq!(
                b"12345678",
                algorithms::$func::<$checked>(12345678u32, &mut buffer)
            );
            assert_buff_eq!(
                b"99999999",
                algorithms::$func::<$checked>(99999999u32, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000",
                algorithms::$func::<$checked>(100000000u32, &mut buffer)
            );
            assert_buff_eq!(
                b"100000001",
                algorithms::$func::<$checked>(100000001u32, &mut buffer)
            );
            assert_buff_eq!(
                b"123456789",
                algorithms::$func::<$checked>(123456789u32, &mut buffer)
            );
            assert_buff_eq!(
                b"999999999",
                algorithms::$func::<$checked>(999999999u32, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000",
                algorithms::$func::<$checked>(1000000000u32, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000001",
                algorithms::$func::<$checked>(1000000001u32, &mut buffer)
            );
            assert_buff_eq!(
                b"2147483647",
                algorithms::$func::<$checked>(2147483647u32, &mut buffer)
            );
            assert_buff_eq!(
                b"2147483648",
                algorithms::$func::<$checked>(2147483648u32, &mut buffer)
            );
            assert_buff_eq!(
                b"4294967295",
                algorithms::$func::<$checked>(4294967295u32, &mut buffer)
            );
            assert_buff_eq!(
                b"4294967295",
                algorithms::$func::<$checked>(-1i32 as u32, &mut buffer)
            );
        }
    };
}

u32_cases!(jeaiii32_original_tests, jeaiii32_original, true);
u32_cases!(jeaiii32_better_tests, jeaiii32_better, true);
u32_cases!(jeaiii32_10_tests, jeaiii32_10, true);
u32_cases!(jeaiii32_10_start_tests, jeaiii32_10_start, true);
u32_cases!(jeaiii32_digits_tests, jeaiii32_digits, true);

fn roundtrip<Func: Fn(u32, &mut [u8]) -> &mut [u8]>(x: u32, cb: Func) -> u32 {
    let mut buffer = [b'\x00'; 16];
    let bytes = cb(x, &mut buffer);
    let string = unsafe { core::str::from_utf8_unchecked(bytes) };
    string.parse().unwrap()
}

macro_rules! u32_roundtrip_cases {
    ($name:ident, $func:ident, $checked:expr) => {
        #[test]
        fn $name() {
            let values: &[u32] = &[
                0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255,
                256, 257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049, 4095, 4096, 4097,
                8191, 8192, 8193, 16383, 16384, 16385, 32767, 32768, 32769, 65535, 65536, 65537,
                131071, 131072, 131073, 262143, 262144, 262145, 524287, 524288, 524289, 1048575,
                1048576, 1048577, 2097151, 2097152, 2097153, 4194303, 4194304, 4194305, 8388607,
                8388608, 8388609, 16777215, 16777216, 16777217, 33554431, 33554432, 33554433,
                67108863, 67108864, 67108865, 134217727, 134217728, 134217729, 268435455,
                268435456, 268435457, 536870911, 536870912, 536870913, 1073741823, 1073741824,
                1073741825, 2147483647, 2147483648, 2147483649, 4294967295,
            ];
            for &i in values.iter() {
                assert_eq!(i, roundtrip(i, algorithms::$func::<$checked>));
            }
        }
    };
}

u32_roundtrip_cases!(jeaiii32_original_roundtrip_tests, jeaiii32_original, true);
u32_roundtrip_cases!(jeaiii32_better_roundtrip_tests, jeaiii32_better, true);
u32_roundtrip_cases!(jeaiii32_10_roundtrip_tests, jeaiii32_10, true);
u32_roundtrip_cases!(jeaiii32_10_start_roundtrip_tests, jeaiii32_10_start, true);
u32_roundtrip_cases!(jeaiii32_digits_roundtrip_tests, jeaiii32_digits, true);

proptest! {
    #[test]
    fn u32_original_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip(i, algorithms::jeaiii32_original::<true>));
    }

    #[test]
    fn u32_better_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip(i, algorithms::jeaiii32_better::<true>));
    }

    #[test]
    fn u32_10_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip(i, algorithms::jeaiii32_10::<true>));
    }

    #[test]
    fn u32_digits_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip(i, algorithms::jeaiii32_digits::<true>));
    }
}
