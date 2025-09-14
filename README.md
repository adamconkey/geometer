# Geometer

Implementations of computational geometry algorithms from scratch in Rust.

![BUILD](https://github.com/adamconkey/computational_geometry/actions/workflows/build.yml/badge.svg)
![TEST](https://github.com/adamconkey/computational_geometry/actions/workflows/tests.yml/badge.svg)
![CLIPPY](https://github.com/adamconkey/computational_geometry/actions/workflows/clippy.yml/badge.svg)

🚧👷‍♂️ **Work In Progress:** This repos is under heavy development right now and just in its nascent stages.

---

## Features 

Currently the algorithms are implemented following Joseph O'Rourke's [Computational Geometry in C](https://www.cambridge.org/core/books/computational-geometry-in-c/22A04E03A4BB10C382A1257F64477E1B). I haven't leaned too much on his code examples as I found I had to structure things differently in Rust, but it's proven a nice introduction to computational geometry so I am using it as my beacon.

### Currently Supported for 2D Polygons
- Area
- Triangulation - $O(n^2)$
- Rotation and translation
- Bounding box
- Convex hull
    - GiftWrapping $O(nh)$ for $h$ hull edges
    - QuickHull $O(nh)$ for $h$ hull edges
    - GrahamScan $O(n \log n)$
    - Incremental $O(n \log n)$
    - Divide and Conquer $O(n \log n)$

### On the Roadmap
- Convex Hull 3D
- Voronoi Diagram
- Animated visualizations of algorithms

---

## Guiding Principles

When you look at or use any of this code, you should take note that it was written with these motivations in mind, currently in this order of priority:

1. **My own learning.** I chose Rust because it's a new language for me that I was really keen on learning. I chose computational geometry becuase it's a new topic for me that I'm interested in learning more about. CG is heavy on algorithms and data structures, so it seemed like a nice framework to explore a new language and learn some cool mathematics along the way.
2. **Others' learning.** I have a vision of being able to provide some high quality visualizations of all of the algorithms in action, so that you can watch in real-time how an algorithm is processing e.g. your mesh or defined polygon. I think this will be a really neat tool to understand the differences between the algorithms, and a vital debugging tool. Stay tuned for this one.
3. **Code redundancy.** You might think redundant code would be a bad thing, I say it's great! When you have multiple implementations that are all trying to produce the same end result but are implemented in different ways, you gain a tremendous amount of confidence in their correctness when they all agree on a wide variety of inputs. That is why you will find 7 implementations of 2D convex hull computation. It also will ultimately aid in (1) and (2) above, providing insights on the algorithm differences and tradeoffs.
4. **Relatively easy to read code.** I point this one out because I might sometimes sacrifice code performance for code readability. I also strive to utilize the Rust language as idiomatically as I can, but take that with a grain of salt as I'm still learning what idiomatic Rust is to begin with.
5. **Great unit tests.** As ever this is a work in progress, but I have found unit tests to be the singlemost crucial thing to have on hand when approaching a refactor. I want to have a robust test suite that instills confidence both in the current code and in any refactors that will inevitably become necessary (I've refactored the entire codebase at least 5 times myself so far). 

--- 

## Visualizer

A visualizer is provided using [rerun.io](https://rerun.io). You must have the `rerun` viewer installed for this to work, you can follow their instructions [here](https://rerun.io/docs/getting-started/installing-viewer#installing-the-viewer). This method worked well for me:
```shell
cargo install rerun-cli --locked
```

Here are some example visualizations (add the `-h` option for further CLI documentation):

```shell
cargo run --features visualizer -- -v triangulation -f interesting_polygon_archive -p skimage_horse
```

![Screen Shot 2025-02-15 at 3 31 57 PM](https://github.com/user-attachments/assets/6b603bd3-c45b-4451-8c40-6cb0f6928105)


```shell
cargo run --features visualizer -- -v convex_hull -f interesting_polygon_archive -p skimage_horse
```

![Screen Shot 2025-03-02 at 11 00 53 AM](https://github.com/user-attachments/assets/1d10839e-d725-48cc-93c0-5031c9af075d)



---

## Benchmarks

Some simple benchmarking capabilities are provided using [Criterion.rs](https://bheisler.github.io/criterion.rs/book/). These are mostly to provide empirical intuition on the runtime of algorithms. Currently only a benchmark for 2D convex hull algorithms is setup, more will be added as more algorithms are implemented.

You can run the benchmarks yourself with
```shell
cargo bench --bench convex_hull
```

Here is a visualization comparing the various implementations:

![Screen Shot 2025-03-09 at 9 56 02 PM](https://github.com/user-attachments/assets/7181b26b-a220-4710-b7c5-f7ca5794f8a5)

This is obviously not super informative, but exists more as a starting infrastructure for benchmarking than a meaningful comparison at this stage.

---
