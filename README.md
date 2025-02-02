# Geometer

Implementations of computational geometry algorithms from scratch in Rust.

![BUILD](https://github.com/adamconkey/computational_geometry/actions/workflows/build.yml/badge.svg)
![TEST](https://github.com/adamconkey/computational_geometry/actions/workflows/tests.yml/badge.svg)
![CLIPPY](https://github.com/adamconkey/computational_geometry/actions/workflows/clippy.yml/badge.svg)

🚧👷‍♂️ **Work In Progress:** This repos is under heavy development right now and just in its nascent stages.

---

Currently the algorithms are implemented following Joseph O'Rourke's [Computational Geometry in C](https://www.cambridge.org/core/books/computational-geometry-in-c/22A04E03A4BB10C382A1257F64477E1B).

This is very much a work in progress, I'm just stepping through the text and implementing things as I go. I'm also a Rust newb so I'm frequently stumbling through the implementations, finding I made a terrible design decision, and going back to reimplement things. As such the API is in constant flux.

My goal for this repo is to eventually have a complete implementation of the algorithms described in the text, which will serve as the basis of a computational geometry library in Rust. I will then build from there, exploring more modern concepts and algorithms. My priorities are to have relatively easy-to-read code, a great test suite, and nice visualizations. I will have these three objectives in mind as I build out this repo.

---

## Features 
### Currently Supported for 2D Polygons
- Area
- Triangulation - $O(n^2)$
- Rotation and translation
- Bounding box computation
- Determination of extreme and interior points - $O(n^3)$ and $O(n^4)$ (for benchmarking)

### On the Roadmap
- Convex Hull (2D and 3D)
- Voronoi Diagram
- Benchmarking of different algorithms
- Animated visualizations of algorithms

---

## Benchmarks

Some simple benchmarking capabilities are provided using [Criterion.rs](https://bheisler.github.io/criterion.rs/book/). These are mostly to provide empirical intuition on the runtime of algorithms. Currently only a couple benchmarks are setup, more will be added as more algorithms are implemented.

You can run the benchmarks yourself with
```shell
cargo bench --bench extreme_points --bench interior_points
```

Here is a visualization of one of the benchmarks to compute extreme points, comparing computing them from extreme edges $O(n^3)$ versus computing them from interior points with triangle checks $O(n^4)$:

![Screen Shot 2025-02-02 at 11 51 21 AM](https://github.com/user-attachments/assets/e6550aec-eac0-4413-b6ec-9fd9526c0ae6)

These are both obviously bad algorithms, but they are what is implemented at the moment and provide some basis for comparison.

---
