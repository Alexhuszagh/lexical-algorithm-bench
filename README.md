# Benchmarks

This implements various algorithms and then quick benchmarks with correctness checks for them. The results are as follows:
- jeaiii algorithm has the fastest performance
- Bounds-checked algorithms play nicely with the jeaiii algorithms
- Working with integer promotion (`u8` -> `u32`) for smaller sizes has faster or equal performance in all cases, probably due to better ease of use in registers
