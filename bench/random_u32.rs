#[macro_use]
mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Default random data size.
const COUNT: usize = 1000;

macro_rules! bench {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let data = input::type_from_random::<u32>($strategy, COUNT, seed);

            write_u32_generator!(group, jeaiii32_original, data.iter(), true);
            write_u32_generator!(group, jeaiii32_better, data.iter(), true);
            write_u32_generator!(group, jeaiii32_digits, data.iter(), true);
            write_u32_generator!(group, jeaiii32_10, data.iter(), true);
            write_u32_generator!(group, jeaiii32_10_start, data.iter(), true);
            write_u32_generator!(group, alexandrescu32, data.iter(), false);
            write_u32_generator!(group, naive_temp32, data.iter(), false);
            write_u32_generator!(group, naive_exact32, data.iter(), false);
            fmt_generator!(group, concat!("write_u32_fmt"), data.iter());
            itoa_generator!(group, concat!("write_u32_itoa"), data.iter());
        }
    };
}

bench!(uniform, "random:uniform", input::RandomGen::Uniform);
bench!(simple, "random:simple", input::RandomGen::Simple);
bench!(large, "random:large", input::RandomGen::Large);
criterion_group!(uniform_benches, uniform);
criterion_group!(simple_benches, simple);
criterion_group!(large_benches, large);
criterion_main!(uniform_benches, simple_benches, large_benches);
