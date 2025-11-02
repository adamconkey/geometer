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
2. **Others' learning.** I have a vision of being able to provide some high quality visualizations of all of the algorithms in action, so that you can watch in real-time how an algorithm is processing e.g. your mesh or defined polygon. I think this will be a really neat tool to understand the differences between the algorithms, and a vital debugging tool.
3. **Code redundancy.** You might think redundant code would be a bad thing, I say it's great! When you have multiple implementations that are all trying to produce the same end result but are implemented in different ways, you gain a tremendous amount of confidence in their correctness when they all agree on a wide variety of inputs. That is why you will find 7 implementations of 2D convex hull computation. It also will ultimately aid in (1) and (2) above, providing insights on the algorithm differences and tradeoffs.
4. **Relatively easy to read code.** I point this one out because I might sometimes sacrifice code performance for code readability. I also strive to utilize the Rust language as idiomatically as I can, but take that with a grain of salt as I'm still learning what idiomatic Rust is to begin with.
5. **Great unit tests.** As ever this is a work in progress, but I have found unit tests to be the singlemost crucial thing to have on hand when approaching a refactor. I want to have a robust test suite that instills confidence both in the current code and in any refactors that will inevitably become necessary (I've refactored the entire codebase at least 5 times myself so far). 

--- 

## Algorithm Visualizer

Any time I learn a new algorithm, good visualizations are key for me to be able to really grasp what's going on. More times than not, such visualizations are not provided, or they are too simplistic or static for them to really shine a light on the confusing bits. As I was implementing the 2D convex hull algorithms, I kept thinking it would be so much easier to understand these if there was just a visualization of them operating on a polygon in action. I thought this so often that I decided to start implementing the visualizations I longed for.

Currently the options are fairly limited, I only have visualizations for 2D convex hull computation using Graham Scan and the Incremental algorithm. Consider them proofs of concept, I will be expanding them to more algorithms and making them more robust. A project for tracking this effort can be found [here](https://github.com/users/adamconkey/projects/4).

The hope is that you watch the visualizations and already start to gain an intuition for how the algorithm might work, and if you look at it in conjunction with reading the algorithm description or looking at the code, it should really start to hit home. Once these visualizations mature, I will build out some better educational content that include more complete algorithm descriptions.

### Setup

The algorithm visualizer uses [rerun.io](https://rerun.io) for the visualizations. You must have the `rerun` viewer installed for this to work, you can follow their instructions [here](https://rerun.io/docs/getting-started/installing-viewer#installing-the-viewer). This method worked well for me:
```shell
cargo install rerun-cli --locked
```

### Examples

Here are some example visualizations:

```shell
cargo run --features visualizer -- -f custom -p polygon_2 -v convex-hull-incremental
```

https://github.com/user-attachments/assets/aed7295e-6c0d-4588-85e9-93e456c5368b


```shell
cargo run --features visualizer -- -f custom -p polygon_2 -v convex-hull-graham-scan
```

https://github.com/user-attachments/assets/20ae578a-7f6a-4a3f-a432-abb42f92d8c3

---

## Benchmarks

Some simple benchmarking capabilities are provided using [Criterion.rs](https://bheisler.github.io/criterion.rs/book/). These are mostly to provide empirical intuition on the runtime of algorithms. Currently only a benchmark for 2D convex hull algorithms is setup, more will be added as more algorithms are implemented.

You can run the benchmarks yourself with
```shell
cargo bench --bench convex_hull
```

Here is a visualization comparing the various implementations:

<img width="941" height="529" alt="Screen Shot 2025-09-14 at 2 10 45 PM" src="https://github.com/user-attachments/assets/eccbd34c-b29f-467a-8f5e-b8df1209db70" />

You'll notice it's a little noisy as it goes across, this is in part due to the benchmark inputs not being particularly well controlled and they are just polygons I happen to have defined. I hope to improve this at some point.

---
