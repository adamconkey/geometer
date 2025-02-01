# Computational Geometry in Rust

![BUILD](https://github.com/adamconkey/computational_geometry/actions/workflows/build.yml/badge.svg)
![TEST](https://github.com/adamconkey/computational_geometry/actions/workflows/tests.yml/badge.svg)
![CLIPPY](https://github.com/adamconkey/computational_geometry/actions/workflows/clippy.yml/badge.svg)


Implementations of computational geometry algorithms from scratch in Rust.

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

Some simple benchmarking capabilities are provided. These are mostly to provide empirical intuition on the runtime of algorithms. Currently only a couple benchmarks are setup, more will be added as more algorithms are implemented.

You can run the benchmarks yourself with
```shell
cargo bench --bench extreme_points interior_points
```

--

## Visualizer

A simple visualizer is provided using the [`egui_plot`](https://github.com/emilk/egui_plot) crate. This provides a local webapp to visualize polygons. Currently this is _very_ simple, and just visualizes the polygons themselves as well as a triangulation. Once more algorithms are implemented I plan to add animations so that one can view the different algorithms in action.

To run the visualizer, simply do:
```bash
cd visualizer
trunk serve
```

You can then direct your browser to `localhost:8080` and you'll hopefully see some polygons! If you're in VSCode, it can be handy to use the `SimpleBrowser` offered in the IDE.

### Polygon Visualization
![Screen Shot 2024-11-23 at 10 48 05 PM](https://github.com/user-attachments/assets/6ebf47f0-57e1-4b9f-9e0f-ffb25827a02c)


### Triangulation Visualization
![Screen Shot 2024-11-23 at 10 47 30 PM](https://github.com/user-attachments/assets/ddeb1724-dde7-4769-b2db-3f48293c4135)

---
