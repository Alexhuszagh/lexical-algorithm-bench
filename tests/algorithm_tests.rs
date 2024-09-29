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
            assert_buff_eq!(b"10000", algorithms::$func::<$checked>(10000u32, &mut buffer));
            assert_buff_eq!(b"10001", algorithms::$func::<$checked>(10001u32, &mut buffer));
            assert_buff_eq!(b"12345", algorithms::$func::<$checked>(12345u32, &mut buffer));
            assert_buff_eq!(b"99999", algorithms::$func::<$checked>(99999u32, &mut buffer));
            assert_buff_eq!(b"100000", algorithms::$func::<$checked>(100000u32, &mut buffer));
            assert_buff_eq!(b"100001", algorithms::$func::<$checked>(100001u32, &mut buffer));
            assert_buff_eq!(b"123456", algorithms::$func::<$checked>(123456u32, &mut buffer));
            assert_buff_eq!(b"999999", algorithms::$func::<$checked>(999999u32, &mut buffer));
            assert_buff_eq!(b"1000000", algorithms::$func::<$checked>(1000000u32, &mut buffer));
            assert_buff_eq!(b"1000001", algorithms::$func::<$checked>(1000001u32, &mut buffer));
            assert_buff_eq!(b"1234567", algorithms::$func::<$checked>(1234567u32, &mut buffer));
            assert_buff_eq!(b"9999999", algorithms::$func::<$checked>(9999999u32, &mut buffer));
            assert_buff_eq!(b"10000000", algorithms::$func::<$checked>(10000000u32, &mut buffer));
            assert_buff_eq!(b"10000001", algorithms::$func::<$checked>(10000001u32, &mut buffer));
            assert_buff_eq!(b"12345678", algorithms::$func::<$checked>(12345678u32, &mut buffer));
            assert_buff_eq!(b"99999999", algorithms::$func::<$checked>(99999999u32, &mut buffer));
            assert_buff_eq!(b"100000000", algorithms::$func::<$checked>(100000000u32, &mut buffer));
            assert_buff_eq!(b"100000001", algorithms::$func::<$checked>(100000001u32, &mut buffer));
            assert_buff_eq!(b"123456789", algorithms::$func::<$checked>(123456789u32, &mut buffer));
            assert_buff_eq!(b"999999999", algorithms::$func::<$checked>(999999999u32, &mut buffer));
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

u32_cases!(naive_exact32_original_tests, naive_exact32, true);
u32_cases!(naive_temp32_original_tests, naive_temp32, true);
u32_cases!(alexandrescu32_tests, alexandrescu32, true);
u32_cases!(jeaiii32_original_tests, jeaiii32_original, true);
u32_cases!(jeaiii32_better_tests, jeaiii32_better, true);
u32_cases!(jeaiii32_10_tests, jeaiii32_10, true);
u32_cases!(jeaiii32_10_start_tests, jeaiii32_10_start, true);
u32_cases!(jeaiii32_digits_tests, jeaiii32_digits, true);

macro_rules! u64_cases {
    ($name:ident, $func:ident, $checked:expr) => {
        #[test]
        fn $name() {
            let mut buffer = [b'\x00'; 32];
            assert_buff_eq!(b"0", algorithms::$func::<$checked>(0u64, &mut buffer));
            assert_buff_eq!(b"1", algorithms::$func::<$checked>(1u64, &mut buffer));
            assert_buff_eq!(b"5", algorithms::$func::<$checked>(5u64, &mut buffer));
            assert_buff_eq!(b"10", algorithms::$func::<$checked>(10u64, &mut buffer));
            assert_buff_eq!(b"11", algorithms::$func::<$checked>(11u64, &mut buffer));
            assert_buff_eq!(b"12", algorithms::$func::<$checked>(12u64, &mut buffer));
            assert_buff_eq!(b"99", algorithms::$func::<$checked>(99u64, &mut buffer));
            assert_buff_eq!(b"100", algorithms::$func::<$checked>(100u64, &mut buffer));
            assert_buff_eq!(b"101", algorithms::$func::<$checked>(101u64, &mut buffer));
            assert_buff_eq!(b"123", algorithms::$func::<$checked>(123u64, &mut buffer));
            assert_buff_eq!(b"999", algorithms::$func::<$checked>(999u64, &mut buffer));
            assert_buff_eq!(b"1000", algorithms::$func::<$checked>(1000u64, &mut buffer));
            assert_buff_eq!(b"1001", algorithms::$func::<$checked>(1001u64, &mut buffer));
            assert_buff_eq!(b"1234", algorithms::$func::<$checked>(1234u64, &mut buffer));
            assert_buff_eq!(b"9999", algorithms::$func::<$checked>(9999u64, &mut buffer));
            assert_buff_eq!(b"10000", algorithms::$func::<$checked>(10000u64, &mut buffer));
            assert_buff_eq!(b"10001", algorithms::$func::<$checked>(10001u64, &mut buffer));
            assert_buff_eq!(b"12345", algorithms::$func::<$checked>(12345u64, &mut buffer));
            assert_buff_eq!(b"99999", algorithms::$func::<$checked>(99999u64, &mut buffer));
            assert_buff_eq!(b"100000", algorithms::$func::<$checked>(100000u64, &mut buffer));
            assert_buff_eq!(b"100001", algorithms::$func::<$checked>(100001u64, &mut buffer));
            assert_buff_eq!(b"123456", algorithms::$func::<$checked>(123456u64, &mut buffer));
            assert_buff_eq!(b"999999", algorithms::$func::<$checked>(999999u64, &mut buffer));
            assert_buff_eq!(b"1000000", algorithms::$func::<$checked>(1000000u64, &mut buffer));
            assert_buff_eq!(b"1000001", algorithms::$func::<$checked>(1000001u64, &mut buffer));
            assert_buff_eq!(b"1234567", algorithms::$func::<$checked>(1234567u64, &mut buffer));
            assert_buff_eq!(b"9999999", algorithms::$func::<$checked>(9999999u64, &mut buffer));
            assert_buff_eq!(b"10000000", algorithms::$func::<$checked>(10000000u64, &mut buffer));
            assert_buff_eq!(b"10000001", algorithms::$func::<$checked>(10000001u64, &mut buffer));
            assert_buff_eq!(b"12345678", algorithms::$func::<$checked>(12345678u64, &mut buffer));
            assert_buff_eq!(b"99999999", algorithms::$func::<$checked>(99999999u64, &mut buffer));
            assert_buff_eq!(b"100000000", algorithms::$func::<$checked>(100000000u64, &mut buffer));
            assert_buff_eq!(b"100000001", algorithms::$func::<$checked>(100000001u64, &mut buffer));
            assert_buff_eq!(b"123456789", algorithms::$func::<$checked>(123456789u64, &mut buffer));
            assert_buff_eq!(b"999999999", algorithms::$func::<$checked>(999999999u64, &mut buffer));
            assert_buff_eq!(
                b"1000000000",
                algorithms::$func::<$checked>(1000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000001",
                algorithms::$func::<$checked>(1000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1234567890",
                algorithms::$func::<$checked>(1234567890u64, &mut buffer)
            );
            assert_buff_eq!(
                b"2147483647",
                algorithms::$func::<$checked>(2147483647u64, &mut buffer)
            );
            assert_buff_eq!(
                b"2147483648",
                algorithms::$func::<$checked>(2147483648u64, &mut buffer)
            );
            assert_buff_eq!(
                b"4294967295",
                algorithms::$func::<$checked>(4294967295u64, &mut buffer)
            );
            assert_buff_eq!(
                b"9999999999",
                algorithms::$func::<$checked>(9999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000000",
                algorithms::$func::<$checked>(10000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000001",
                algorithms::$func::<$checked>(10000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"12345678901",
                algorithms::$func::<$checked>(12345678901u64, &mut buffer)
            );
            assert_buff_eq!(
                b"99999999999",
                algorithms::$func::<$checked>(99999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000000",
                algorithms::$func::<$checked>(100000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000001",
                algorithms::$func::<$checked>(100000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"123456789012",
                algorithms::$func::<$checked>(123456789012u64, &mut buffer)
            );
            assert_buff_eq!(
                b"999999999999",
                algorithms::$func::<$checked>(999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000000",
                algorithms::$func::<$checked>(1000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000001",
                algorithms::$func::<$checked>(1000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1234567890123",
                algorithms::$func::<$checked>(1234567890123u64, &mut buffer)
            );
            assert_buff_eq!(
                b"9999999999999",
                algorithms::$func::<$checked>(9999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000000000",
                algorithms::$func::<$checked>(10000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000000001",
                algorithms::$func::<$checked>(10000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"12345678901234",
                algorithms::$func::<$checked>(12345678901234u64, &mut buffer)
            );
            assert_buff_eq!(
                b"99999999999999",
                algorithms::$func::<$checked>(99999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000000000",
                algorithms::$func::<$checked>(100000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000000001",
                algorithms::$func::<$checked>(100000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"123456789012345",
                algorithms::$func::<$checked>(123456789012345u64, &mut buffer)
            );
            assert_buff_eq!(
                b"999999999999999",
                algorithms::$func::<$checked>(999999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000000000",
                algorithms::$func::<$checked>(1000000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000000001",
                algorithms::$func::<$checked>(1000000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1234567890123456",
                algorithms::$func::<$checked>(1234567890123456u64, &mut buffer)
            );
            assert_buff_eq!(
                b"9999999999999999",
                algorithms::$func::<$checked>(9999999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000000000000",
                algorithms::$func::<$checked>(10000000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"10000000000000001",
                algorithms::$func::<$checked>(10000000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"12345678901234567",
                algorithms::$func::<$checked>(12345678901234567u64, &mut buffer)
            );
            assert_buff_eq!(
                b"99999999999999999",
                algorithms::$func::<$checked>(99999999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000000000000",
                algorithms::$func::<$checked>(100000000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"100000000000000001",
                algorithms::$func::<$checked>(100000000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"123456789012345678",
                algorithms::$func::<$checked>(123456789012345678u64, &mut buffer)
            );
            assert_buff_eq!(
                b"999999999999999999",
                algorithms::$func::<$checked>(999999999999999999u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000000000000",
                algorithms::$func::<$checked>(1000000000000000000u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1000000000000000001",
                algorithms::$func::<$checked>(1000000000000000001u64, &mut buffer)
            );
            assert_buff_eq!(
                b"1234567890123456789",
                algorithms::$func::<$checked>(1234567890123456789u64, &mut buffer)
            );
            assert_buff_eq!(
                b"9223372036854775807",
                algorithms::$func::<$checked>(9223372036854775807u64, &mut buffer)
            );
            assert_buff_eq!(
                b"9223372036854775808",
                algorithms::$func::<$checked>(9223372036854775808u64, &mut buffer)
            );
            assert_buff_eq!(
                b"18446744073709551615",
                algorithms::$func::<$checked>(18446744073709551615u64, &mut buffer)
            );
            assert_buff_eq!(
                b"18446744073709551615",
                algorithms::$func::<$checked>(-1i64 as u64, &mut buffer)
            );
        }
    };
}

u64_cases!(naive_exact64_tests, naive_exact64, true);
u64_cases!(naive_temp64_tests, naive_temp64, true);
u64_cases!(alexandrescu64_tests, alexandrescu64, true);
u64_cases!(jeaiii64_better_tests, jeaiii64_better, true);

fn roundtrip_u32<Func: Fn(u32, &mut [u8]) -> &mut [u8]>(x: u32, cb: Func) -> u32 {
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
                assert_eq!(i, roundtrip_u32(i, algorithms::$func::<$checked>));
            }
        }
    };
}

fn roundtrip_u64<Func: Fn(u64, &mut [u8]) -> &mut [u8]>(x: u64, cb: Func) -> u64 {
    let mut buffer = [b'\x00'; 32];
    let bytes = cb(x, &mut buffer);
    let string = unsafe { core::str::from_utf8_unchecked(bytes) };
    string.parse().unwrap()
}

macro_rules! u64_roundtrip_cases {
    ($name:ident, $func:ident, $checked:expr) => {
        #[test]
        fn $name() {
            let values: &[u64] = &[
                0,
                1,
                2,
                3,
                4,
                5,
                7,
                8,
                9,
                15,
                16,
                17,
                31,
                32,
                33,
                63,
                64,
                65,
                127,
                128,
                129,
                255,
                256,
                257,
                511,
                512,
                513,
                1023,
                1024,
                1025,
                2047,
                2048,
                2049,
                4095,
                4096,
                4097,
                8191,
                8192,
                8193,
                16383,
                16384,
                16385,
                32767,
                32768,
                32769,
                65535,
                65536,
                65537,
                131071,
                131072,
                131073,
                262143,
                262144,
                262145,
                524287,
                524288,
                524289,
                1048575,
                1048576,
                1048577,
                2097151,
                2097152,
                2097153,
                4194303,
                4194304,
                4194305,
                8388607,
                8388608,
                8388609,
                16777215,
                16777216,
                16777217,
                33554431,
                33554432,
                33554433,
                67108863,
                67108864,
                67108865,
                134217727,
                134217728,
                134217729,
                268435455,
                268435456,
                268435457,
                536870911,
                536870912,
                536870913,
                1073741823,
                1073741824,
                1073741825,
                2147483647,
                2147483648,
                2147483649,
                4294967295,
                4294967296,
                4294967297,
                8589934591,
                8589934592,
                8589934593,
                17179869183,
                17179869184,
                17179869185,
                34359738367,
                34359738368,
                34359738369,
                68719476735,
                68719476736,
                68719476737,
                137438953471,
                137438953472,
                137438953473,
                274877906943,
                274877906944,
                274877906945,
                549755813887,
                549755813888,
                549755813889,
                1099511627775,
                1099511627776,
                1099511627777,
                2199023255551,
                2199023255552,
                2199023255553,
                4398046511103,
                4398046511104,
                4398046511105,
                8796093022207,
                8796093022208,
                8796093022209,
                17592186044415,
                17592186044416,
                17592186044417,
                35184372088831,
                35184372088832,
                35184372088833,
                70368744177663,
                70368744177664,
                70368744177665,
                140737488355327,
                140737488355328,
                140737488355329,
                281474976710655,
                281474976710656,
                281474976710657,
                562949953421311,
                562949953421312,
                562949953421313,
                1125899906842623,
                1125899906842624,
                1125899906842625,
                2251799813685247,
                2251799813685248,
                2251799813685249,
                4503599627370495,
                4503599627370496,
                4503599627370497,
                9007199254740991,
                9007199254740992,
                9007199254740993,
                18014398509481983,
                18014398509481984,
                18014398509481985,
                36028797018963967,
                36028797018963968,
                36028797018963969,
                72057594037927935,
                72057594037927936,
                72057594037927937,
                144115188075855871,
                144115188075855872,
                144115188075855873,
                288230376151711743,
                288230376151711744,
                288230376151711745,
                576460752303423487,
                576460752303423488,
                576460752303423489,
                1152921504606846975,
                1152921504606846976,
                1152921504606846977,
                2305843009213693951,
                2305843009213693952,
                2305843009213693953,
                4611686018427387903,
                4611686018427387904,
                4611686018427387905,
                9223372036854775807,
                9223372036854775808,
                9223372036854775809,
                18446744073709551615,
            ];
            for &i in values.iter() {
                assert_eq!(i, roundtrip_u64(i, algorithms::$func::<$checked>));
            }
        }
    };
}

u32_roundtrip_cases!(naive_temp32_roundtrip_tests, naive_temp32, true);
u32_roundtrip_cases!(naive_exact32_roundtrip_tests, naive_exact32, true);
u32_roundtrip_cases!(alexandrescu32_roundtrip_tests, alexandrescu32, true);
u32_roundtrip_cases!(jeaiii32_original_roundtrip_tests, jeaiii32_original, true);
u32_roundtrip_cases!(jeaiii32_better_roundtrip_tests, jeaiii32_better, true);
u32_roundtrip_cases!(jeaiii32_10_roundtrip_tests, jeaiii32_10, true);
u32_roundtrip_cases!(jeaiii32_10_start_roundtrip_tests, jeaiii32_10_start, true);
u32_roundtrip_cases!(jeaiii32_digits_roundtrip_tests, jeaiii32_digits, true);

u64_roundtrip_cases!(naive_temp64_roundtrip_tests, naive_temp64, true);
u64_roundtrip_cases!(naive_exact64_roundtrip_tests, naive_exact64, true);
u64_roundtrip_cases!(alexandrescu64_roundtrip_tests, alexandrescu64, true);
u64_roundtrip_cases!(jeaiii64_better_roundtrip_tests, jeaiii64_better, true);

proptest! {
    #[test]
    fn naive_temp32_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::naive_temp32::<true>));
    }

    #[test]
    fn naive_exact32_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::naive_exact32::<true>));
    }

    #[test]
    fn alexandrescu32_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::alexandrescu32::<true>));
    }

    #[test]
    fn u32_original_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::jeaiii32_original::<true>));
    }

    #[test]
    fn u32_better_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::jeaiii32_better::<true>));
    }

    #[test]
    fn u32_10_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::jeaiii32_10::<true>));
    }

    #[test]
    fn u32_digits_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip_u32(i, algorithms::jeaiii32_digits::<true>));
    }

    #[test]
    fn naive_temp64_proptest(i in u64::MIN..u64::MAX) {
        prop_assert_eq!(i, roundtrip_u64(i, algorithms::naive_temp64::<true>));
    }

    #[test]
    fn naive_exact64_proptest(i in u64::MIN..u64::MAX) {
        prop_assert_eq!(i, roundtrip_u64(i, algorithms::naive_exact64::<true>));
    }

    #[test]
    fn alexandrescu64_proptest(i in u64::MIN..u64::MAX) {
        prop_assert_eq!(i, roundtrip_u64(i, algorithms::alexandrescu64::<true>));
    }

    #[test]
    fn u64_better_proptest(i in u64::MIN..u64::MAX) {
        prop_assert_eq!(i, roundtrip_u64(i, algorithms::jeaiii64_better::<true>));
    }
}
