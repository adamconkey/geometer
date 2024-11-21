# Computational Geometry in Rust

![TEST](https://github.com/adamconkey/computational_geometry/actions/workflows/tests.yml/badge.svg)

Repo for playing around with implementing computational geometry algorithms from scratch in Rust.

Currently the algorithms are implemented following Joseph O'Rourke's [Computational Geometry in C](https://www.cambridge.org/core/books/computational-geometry-in-c/22A04E03A4BB10C382A1257F64477E1B).

This is very much a work in progress, I'm just stepping through the text and implementing things as I go. I'm also a Rust newb so I'm frequently stumbling through the implementations, finding I made a terrible design decision, and going back to reimplement things. As such the API is in constant flux.

My goal for this repo is to eventually have a complete implementation of the algorithms described in the text, which will serve as the basis of a computational geometry library in Rust. I will then build from there, exploring more modern concepts and algorithms. My priorities are to have relatively easy-to-read code, a great test suite, and nice visualizations. I will have these three objectives in mind as I build out this repo.

## Running the Visualizer

A simple visualizer is provided using the [`egui_plot`](https://github.com/emilk/egui_plot) crate. This provides a local webapp to visualize polygons. Currently this is _very_ simple, and just visualizes the polygons themselves. I have plans to support visualizations of results from the tests.

To run the visualizer, simply do:
```bash
cd visualizer
trunk serve
```

You can then direct your browser to `localhost:8080` and you'll hopefully see some polygons! If you're in VSCode, it can be handy to use the `SimpleBrowser` offered in the IDE:

![Screen Shot 2024-11-20 at 8 17 22 PM](https://github.com/user-attachments/assets/013225ce-f524-4a37-9cec-46643451858e)
