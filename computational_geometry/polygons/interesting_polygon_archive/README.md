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


|     |     |     |
|:---:|:---:|:---:|
| **Eberly 10** | **Eberly 14** | **Elgindy 1** | 
| ![eberly-10](https://github.com/user-attachments/assets/46985dc4-133b-4b36-b441-33e6e2bae5ec) | ![eberly-14](https://github.com/user-attachments/assets/a6b28940-00c9-402d-a2a6-63150c868d5c) | ![elgindy-1](https://github.com/user-attachments/assets/4a1c45f8-8607-45fb-b5e6-e0e5ea8e157a) |
| **Gray Embroidery** | **Held 1** | **Held 3** |
| ![gray-embroidery](https://github.com/user-attachments/assets/f7cb6cc3-9930-4254-a8e1-0c7c68b499b8) | ![held-1](https://github.com/user-attachments/assets/98da11b7-9af1-4e1a-aace-03ac2a9ee8ff) | ![held-3](https://github.com/user-attachments/assets/03137048-6d50-4310-81fb-e1f0c9df1c49) |
| **Held 7a** | **Held 7b** | **Held 7c** |
| ![held-7a](https://github.com/user-attachments/assets/4b635066-48ee-44cc-8656-f0d084f7c31e) | ![held-7b](https://github.com/user-attachments/assets/967dd001-f951-4354-a5bb-d1c836d660f6) | ![held-7c](https://github.com/user-attachments/assets/45510876-0387-4905-969a-f03960d9544b) |

![toussaint-1a](https://github.com/user-attachments/assets/bc6ba928-dd3e-4ca6-8221-dc4619ad5c62)
![skimage-horse](https://github.com/user-attachments/assets/47ab5f7b-caea-4465-a862-c201f5de2f06)
![seidel-3](https://github.com/user-attachments/assets/8b102403-6eba-493d-b3fd-1ed561f2331e)
![Screen Shot 2025-01-11 at 9 20 17 PM](https://github.com/user-attachments/assets/ed317703-71d7-43d0-be73-5abac1e333bf)
![misc-fu](https://github.com/user-attachments/assets/84fd8bbf-3203-46e3-9b8b-5c03424b781b)
![misc-discobolus](https://github.com/user-attachments/assets/fb0c1ee1-3db5-4916-9ca9-3e28cc155e8d)
![meisters-3](https://github.com/user-attachments/assets/1b6672ed-788f-4601-befb-bbec0b9d8c6d)
![mei-6](https://github.com/user-attachments/assets/379d9ba7-8e8c-41a3-bd13-f20039e38220)
![mei-5](https://github.com/user-attachments/assets/472dfff5-c954-4661-9759-a8dac72fcf0e)
![mei-4](https://github.com/user-attachments/assets/f489f09c-be84-4964-81eb-5d1058ac0377)
![mei-3](https://github.com/user-attachments/assets/d67d07f0-e571-4ca9-98b0-20fd2f08c9b3)
![mei-2](https://github.com/user-attachments/assets/050e137e-072f-4b79-93b2-2d17198359b6)
![matisse-nuit](https://github.com/user-attachments/assets/29f68654-ef4b-4829-9f1c-724f91b7d06d)
![matisse-icarus](https://github.com/user-attachments/assets/940b84f0-7642-40fa-aa4f-071691f4a130)
![matisse-blue](https://github.com/user-attachments/assets/e0ed1abb-d79b-475d-8d01-c2d769181aab)
![matisse-alga](https://github.com/user-attachments/assets/74d0c35a-4c48-4b6f-b3e2-ca5a0a9259ff)
![mapbox-dude](https://github.com/user-attachments/assets/06ecd318-3d50-4999-841b-aa570e566c83)
![mapbox-building](https://github.com/user-attachments/assets/81d499d2-e558-440e-9146-f6e2323ad1ea)
![held-12](https://github.com/user-attachments/assets/f177dc69-1072-406b-9d2b-5df03d22c147)
![held-7d](https://github.com/user-attachments/assets/675aa84b-df07-4768-aa35-aebfa75fde00)




![toussaint-1a](https://github.com/user-attachments/assets/dbc6ea22-ef21-4269-9cc4-52b4e0dff2a7)


## Running the Visualizer

You may view these polygons locally running the visualizer, from the directory `$REPO_ROOT/visualizer` run
```bash
trunk serve
```
and if you open your browser to `localhost:8080` you'll be able to browse them. 
