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

            let data = input::type_from_random::<u16>($strategy, COUNT, seed);

            write_u16_generator!(group, jeaiii16_better, data.iter());
            write_u16_generator!(group, jeaiii16as32_better, data.iter());
            write_u16_generator!(group, alexandrescu16, data.iter());
            write_u16_generator!(group, naive_temp16, data.iter());
            write_u16_generator!(group, naive_exact16, data.iter());
            fmt_generator!(group, "write_u16_fmt", data.iter());
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
