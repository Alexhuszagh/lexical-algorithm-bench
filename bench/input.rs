//! Input data reader and random-number generator for benchmarks.
//! This is adapted from fast-float-rust.

// `unused_macro_rules` isn't known until nightly-2022-05-12
#![allow(dead_code, unused_macros, unknown_lints, unused_macro_rules)]

use core::fmt::Debug;
use core::str::FromStr;

use fastrand::Rng;

pub(crate) const BUFFER_SIZE: usize = 144;

// PATH

/// Return the `target` directory path.
#[inline]
pub fn target_dir() -> std::path::PathBuf {
    // Cross-compiling creates a different directory
    let mut path = std::env::current_exe().unwrap();
    // TODO: Fix the data directories
    while let Some(basename) = path.file_name() {
        if basename == "target" {
            break;
        } else {
            path.pop();
        }
    }

    path
}

/// Return the benchmark directory path.
#[inline]
pub fn bench_dir() -> std::path::PathBuf {
    let mut path = target_dir();
    path.pop();
    path
}

// FILE

/// Parse JSON data from file.
#[inline]
pub fn read_json<T: serde::de::DeserializeOwned>(name: &str) -> T {
    let mut path = bench_dir();
    path.push("data");
    path.push(name);
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

/// Read data as lines from file.
#[inline]
pub fn read_lines(name: &str) -> Vec<String> {
    let mut path = bench_dir();
    path.push("data");
    path.push(name);
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

/// Read data as CSV from file.
#[inline]
pub fn read_csv(name: &str) -> Vec<String> {
    let mut path = bench_dir();
    path.push("data");
    path.push(name);
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .flat_map(|x| x.split(','))
        .map(String::from)
        .collect()
}

/// Parse JSON data as a given type.
macro_rules! json_data {
    ($t:ty, $file:literal) => {
        fn json_data() -> &'static $t {
            use lazy_static::lazy_static;
            lazy_static! {
                static ref DATA: $t = input::read_json($file);
            }
            &*DATA
        }
    };
}

/// Generate an array of values as static data
///
/// - `fn` - The name to register the static data as
/// - `cb` - The function to fetch the data
/// - `f1` - The field within the data to fetch
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! static_data {
    ($($fn:ident $cb:ident $f1:ident $t:tt ; )*) => ($(
        fn $fn() -> &'static [$t] {
            use lazy_static::lazy_static;
            lazy_static! {
                static ref DATA: Vec<$t> = {
                    $cb()
                        .$f1
                        .iter()
                        .map(|x| x.parse::<$t>().unwrap())
                        .collect()
                };
            }
            &*DATA
        }
    )*);

    ($($fn:ident $cb:ident $f1:ident $f2:ident $t:tt ; )*) => ($(
        fn $fn() -> &'static [$t] {
            use lazy_static::lazy_static;
            lazy_static! {
                static ref DATA: Vec<$t> = {
                    $cb()
                        .$f1
                        .$f2
                        .iter()
                        .map(|x| x.parse::<$t>().unwrap())
                        .collect()
                };
            }
            &*DATA
        }
    )*);
}

// RANDOM

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RandomGen {
    // Generic
    Uniform,

    // Integers
    Simple,
    SimpleSigned,
    Large,
    LargeSigned,

    // Floats
    OneOverRand32,
    SimpleUniform32,
    SimpleInt32,
    IntEInt,
    SimpleInt64,
    BigIntDotInt,
    BigInts,
}

pub trait NumberRng: Sized + ToString {
    fn gen(strategy: RandomGen, rng: &mut Rng) -> String;
}

pub trait IntegerRng: NumberRng {
    fn uniform(rng: &mut Rng) -> String;
    fn simple(rng: &mut Rng) -> String;
    fn large(rng: &mut Rng) -> String;
    fn simple_signed(rng: &mut Rng) -> String;
    fn large_signed(rng: &mut Rng) -> String;
}

/// Generate an unsigned, random range for testing.
///
/// - `min` - The min for simple values
/// - `max` - The max for simple values
/// - `lmin` - The min for large values
/// - `lmax` - The max for large values
macro_rules! unsigned_rng {
    ($($t:ident $smin:literal $smax:literal $lmin:literal $lmax:literal ; )*) => ($(
        impl NumberRng for $t {
            fn gen(strategy: RandomGen, rng: &mut Rng) -> String {
                match strategy {
                    RandomGen::Uniform => Self::uniform(rng),
                    RandomGen::Simple => Self::simple(rng),
                    RandomGen::SimpleSigned => Self::simple_signed(rng),
                    RandomGen::Large => Self::large(rng),
                    RandomGen::LargeSigned => Self::large_signed(rng),
                    _ => unimplemented!(),
                }
            }
        }

        impl IntegerRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> String {
                (rng.$t(<$t>::MIN..<$t>::MAX)).to_string()
            }

            #[inline]
            fn simple(rng: &mut Rng) -> String {
                (rng.$t($smin..$smax)).to_string()
            }

            #[inline]
            fn simple_signed(_: &mut Rng) -> String {
                unimplemented!()
            }

            #[inline]
            fn large(rng: &mut Rng) -> String {
                (rng.$t($lmin..$lmax)).to_string()
            }

            #[inline]
            fn large_signed(_: &mut Rng) -> String {
                unimplemented!()
            }
        }
    )*);
}

/// Generate a signed, random range for testing.
///
/// - `smin` - The min for simple values
/// - `smax` - The max for simple values
/// - `ssmin` - The min for signed, simple values
/// - `ssmax` - The max for signed, simple values
/// - `lmin` - The min for large values
/// - `lmax` - The max for large values
/// - `lsmin` - The min for signed, large values
/// - `lsmax` - The max for signed, large values
macro_rules! signed_rng {
    ($(
        $t:ident
        $smin:literal $smax:literal $lmin:literal $lmax:literal
        $ssmin:literal $ssmax:literal $lsmin:literal $lsmax:literal
        ;
    )*) => ($(
        impl NumberRng for $t {
            fn gen(strategy: RandomGen, rng: &mut Rng) -> String {
                match strategy {
                    RandomGen::Uniform => Self::uniform(rng),
                    RandomGen::Simple => Self::simple(rng),
                    RandomGen::SimpleSigned => Self::simple_signed(rng),
                    RandomGen::Large => Self::large(rng),
                    RandomGen::LargeSigned => Self::large_signed(rng),
                    _ => unimplemented!(),
                }
            }
        }

        impl IntegerRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> String {
                (rng.$t(<$t>::MIN..<$t>::MAX)).to_string()
            }

            #[inline]
            fn simple(rng: &mut Rng) -> String {
                (rng.$t($smin..$smax)).to_string()
            }

            #[inline]
            fn simple_signed(rng: &mut Rng) -> String {
                (rng.$t($ssmin..$ssmax)).to_string()
            }

            #[inline]
            fn large(rng: &mut Rng) -> String {
                (rng.$t($lmin..$lmax)).to_string()
            }

            #[inline]
            fn large_signed(rng: &mut Rng) -> String {
                (rng.$t($lsmin..$lsmax)).to_string()
            }
        }
    )*);
}

unsigned_rng! {
    u8 0 50 100 255 ;
    u16 0 1000 1024 65535 ;
    u32 0 1000 67108864 4294967295 ;
    u64 0 1000 288230376151711744 18446744073709551615 ;
    u128 0 1000 5316911983139663491615228241121378304 340282366920938463463374607431768211455 ;
}

signed_rng! {
    i8 0 50 100 127 -50 50 -127 -100 ;
    i16 0 1000 1024 32767 -1000 1000 -32767 -1024 ;
    i32 0 1000 67108864 2147483647 -1000 1000 -2147483647 -67108864 ;
    i64 0 1000 288230376151711744 9223372036854775807 -1000 1000 -9223372036854775807 -288230376151711744 ;
    i128 0 1000 5316911983139663491615228241121378304 170141183460469231731687303715884105727 -1000 1000 -170141183460469231731687303715884105727 -5316911983139663491615228241121378304 ;
}

// Generate a static array of random values.
#[inline]
pub fn string_from_random<T>(strategy: RandomGen, count: usize, seed: u64) -> Vec<String>
where
    T: NumberRng,
{
    let mut rng = Rng::with_seed(seed);
    let mut vec: Vec<String> = Vec::with_capacity(count);
    for _ in 0..count {
        vec.push(T::gen(strategy, &mut rng));
    }
    vec
}

// Generate a static array of random values.
#[inline]
pub fn type_from_random<T>(strategy: RandomGen, count: usize, seed: u64) -> Vec<T>
where
    T: NumberRng + FromStr,
    <T as FromStr>::Err: Debug,
{
    string_from_random::<T>(strategy, count, seed)
        .iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

// GENERATORS

// For all of these:
// - `group`: The name of the group containing mutiple benches.
// - `name`: The name of the bench within the group.
// - `iter`: An abstract iterable over the data to process.

macro_rules! checked_generator {
    ($group:ident, $name:expr, $iter:expr, $func:ident, $checked:literal) => {{
        use crate::input::BUFFER_SIZE;
        let mut buffer: [u8; BUFFER_SIZE] = [b'0'; BUFFER_SIZE];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|&x| {
                    black_box(algorithms::$func::<$checked>(x, &mut buffer));
                })
            })
        });
    }};
}

macro_rules! fmt_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        use std::io::Write;

        use crate::input::BUFFER_SIZE;
        let mut buffer = vec![b'0'; BUFFER_SIZE];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|&x| {
                    black_box(buffer.write_fmt(format_args!("{}", x)).unwrap());
                    unsafe {
                        buffer.set_len(0);
                    }
                })
            })
        });
    }};
}

macro_rules! write_u8_generator {
    ($group:ident, $type:expr, $iter:expr) => {{
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_b_c"),
            $iter,
            jeaiii8_better,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_b_u"),
            $iter,
            jeaiii8_better,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_as32_b_c"),
            $iter,
            jeaiii8as32_better,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_as32_b_u"),
            $iter,
            jeaiii8as32_better,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_alexandrescu_c"),
            $iter,
            alexandrescu8,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_alexandrescu_u"),
            $iter,
            alexandrescu8,
            false
        );
    }};
}

macro_rules! write_u16_generator {
    ($group:ident, $type:expr, $iter:expr) => {{
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_b_c"),
            $iter,
            jeaiii16_better,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_b_u"),
            $iter,
            jeaiii16_better,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_as32_b_c"),
            $iter,
            jeaiii16as32_better,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_as32_b_u"),
            $iter,
            jeaiii16as32_better,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_alexandrescu_c"),
            $iter,
            alexandrescu16,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_alexandrescu_u"),
            $iter,
            alexandrescu16,
            false
        );
    }};
}

macro_rules! write_u32_generator {
    ($group:ident, $type:expr, $iter:expr) => {{
        checked_generator!(
            $group,
            concat!("write_", $type, "_naive_t_c"),
            $iter,
            naive32_temp,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_naive_t_u"),
            $iter,
            naive32_temp,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_naive_e_c"),
            $iter,
            naive32_exact,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_naive_e_u"),
            $iter,
            naive32_exact,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_alexandrescu_c"),
            $iter,
            alexandrescu32,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_alexandrescu_u"),
            $iter,
            alexandrescu32,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_o_c"),
            $iter,
            jeaiii32_original,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_o_u"),
            $iter,
            jeaiii32_original,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_b_c"),
            $iter,
            jeaiii32_better,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_b_u"),
            $iter,
            jeaiii32_better,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_bd_c"),
            $iter,
            jeaiii32_digits,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_bd_u"),
            $iter,
            jeaiii32_digits,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_10_c"),
            $iter,
            jeaiii32_10,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_10_u"),
            $iter,
            jeaiii32_10,
            false
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_10_start_c"),
            $iter,
            jeaiii32_10_start,
            true
        );
        checked_generator!(
            $group,
            concat!("write_", $type, "_jeaiii_10_start_u"),
            $iter,
            jeaiii32_10_start,
            false
        );
        fmt_generator!($group, concat!("write_", $type, "_fmt"), $iter);
    }};
}
