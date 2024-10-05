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

            let data = input::type_from_random::<u64>($strategy, COUNT, seed), true;

            write_u64_generator!(group, jeaiii64_better, data.iter(), true);
            write_u64_generator!(group, jeaiii64_better_v2, data.iter(), true);
            write_u64_generator!(group, jeaiii64_better_v3, data.iter(), true);
            write_u64_generator!(group, jeaiii64_better_v4, data.iter(), true);
            write_u64_generator!(group, jeaiii64_better_v5, data.iter(), true);
            write_u64_generator!(group, jeaiii64_better_v6, data.iter(), true);
            write_u64_generator!(group, alexandrescu64, data.iter(), false);
            write_u64_generator!(group, naive_temp64, data.iter(), false);
            write_u64_generator!(group, naive_exact64, data.iter(), false);
            fmt_generator!(group, concat!("write_u64_fmt"), data.iter());
            itoa_generator!(group, concat!("write_u64_itoa"), data.iter());
        }
    };
}

bench!(uniform, "random:uniform", input::RandomGen::Uniform);
bench!(simple, "random:simple", input::RandomGen::Simple);
bench!(large, "random:large", input::RandomGen::Large);
bench!(safe_int, "random:safe_int", input::RandomGen::SafeInt);
bench!(large_safe_int, "random:large_safe_int", input::RandomGen::LargeSafeInt);
criterion_group!(uniform_benches, uniform);
criterion_group!(simple_benches, simple);
criterion_group!(large_benches, large);
criterion_group!(safe_int_benches, safe_int);
criterion_group!(large_safe_int_benches, large_safe_int);
criterion_main!(uniform_benches, simple_benches, large_benches, safe_int_benches, large_safe_int_benches);
