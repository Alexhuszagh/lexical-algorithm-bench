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

            let data = input::type_from_random::<u64>($strategy, COUNT, seed);

            write_u64_generator!(group, jeaiii64_better, data.iter());
            write_u64_generator!(group, jeaiii64_better_v2, data.iter());
            write_u64_generator!(group, alexandrescu64, data.iter());
            write_u64_generator!(group, naive_temp64, data.iter());
            write_u64_generator!(group, naive_exact64, data.iter());
            fmt_generator!(group, concat!("write_u64_fmt"), data.iter());
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
