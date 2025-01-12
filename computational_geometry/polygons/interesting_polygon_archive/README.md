# Interesting Polygon Archive (IPA)

These polygons are more or less copied from the [Interesting Polygon Archive](https://github.com/LingDong-/interesting-polygon-archive) which was supported by the [Frank-Ratchye Studio for Creative Inquiry](https://studioforcreativeinquiry.org/) at Carnegie Mellon University. Please see their repo for more information.

## Differences from IPA Dataset

I made a few modifications to make the definitions suitable for this library. I copied the points from the [JSON directory](https://github.com/LingDong-/interesting-polygon-archive/tree/master/json) and asked ChatGPT to format them into the JSON formatted expected by this library.

Polygons with holes are not yet supported by this repo, but the IPA dataset had several with holes. I adopted the practice of taking only the outer boundary from the IPA polygons if they had holes (the first line of points in their JSON files). This means some of them look a little silly. When this repo supports holes, I will circle back and fix them.

I discovered they were upside down because the IPA dataset assumed a the +Y axis was pointing down, while I assume it's pointing up in my visualizer. So, I rotated all of them 180 degrees about the origin and translated them back to the first quadrant of the plot. I realize this makes them face a different direction than the IPA dataset, but I'm okay with that.

The original (ChatGPT-created) JSON files are in the `original` subdirectory. The binary [`generate_rotated_ipa_polgyons.rs`](https://github.com/adamconkey/computational_geometry/blob/19d5541c8bc7b508508bbf1a61b6dd4e76d755ad/computational_geometry/src/bin/generate_rotated_ipa_polygons.rs) was used to generate the files used for the unit tests and visualizations in this repo. You can run that script from within the `computational_geometry` package with:
```shell
cargo run --bin generate-rotated-ipa-polygons
```

## Polygons

## Running the Visualizer

You may view these polygons locally running the visualizer, from the directory `$REPO_ROOT/visualizer` run
```bash
trunk serve
```
and if you open your browser to `localhost:8080` you'll be able to browse them. 