# Benchmarks

This implements various algorithms and then quick benchmarks with correctness checks for them. The results are as follows:
- jeaiii algorithm has the fastest performance
- Bounds-checked algorithms play nicely with the jeaiii algorithms
- Working with integer promotion (`u8` -> `u32`) for smaller sizes has faster or equal performance in all cases, probably due to better ease of use in registers
  - This also is way easier because it makes the implementation simpler

## Best

**<= 32 Bits:**

`write_u*_jeaiii_as32_b_c` are the best algorithms. These convert a <= 32-bit integer to 32-bit integer and use the `write_u32_jeaiii_b_c` algorithm to write the integer. The `_c` means checked, which consistently has identical or superior performance.

For fixed-width, `write_u32_jeaiii_10_c` is ~4x faster than `write_u32_jeaiii_b_c`, however, using a memcopy to align it to the start is extremely so.

**64 Bits:**

Oddly enough, using a standard approach with always writing 64 digits seems to fail miserably. Using standard division with `write_u32_jeaiii_b_c` as follows works for simple floats but not larger ones:

```rust
#[inline(always)]
pub fn jeaiii64_better<const CHECKED: bool>(n: u64, buffer: &mut [u8]) -> &mut [u8] {
    const U32_MAX: u64 = u32::MAX as u64;
    const FACTOR: u64 = 10_0000_0000;
    if n <= U32_MAX {
        jeaiii32_better::<CHECKED>(n as u32, buffer)
    } else if n <= U32_MAX * FACTOR {
        let hi = (n / FACTOR) as u32;
        let lo = (n % FACTOR) as u32;
        ...
    } else {
        let mid = n / FACTOR;
        let hi = (mid / FACTOR) as u32;
        let mid = ((mid) % FACTOR) as u32;
        let lo = (n % FACTOR) as u32;
        ...
    }
}
```

It seems a lot of the performance issues come from having 3 branches and not 2: creating an incorrect algorithm without the last branch has significantly faster performance.
