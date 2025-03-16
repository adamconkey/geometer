# Interesting Polygon Archive (IPA)

These polygons are more or less copied from the [Interesting Polygon Archive](https://github.com/LingDong-/interesting-polygon-archive) which was supported by the [Frank-Ratchye Studio for Creative Inquiry](https://studioforcreativeinquiry.org/) at Carnegie Mellon University. Please see their repo for more information.

## Polygons

These are visualizations of triangulations generated for each polygon (see [Running the Visualizer](#running-the-visualizer) below for how to view these locally yourself with this library's visualizer).

|     |     |     |
|:---:|:---:|:---:|
| **Eberly 10** | **Eberly 14** | **Elgindy 1** |
| ![eberly-10](https://github.com/user-attachments/assets/fd3bdd32-f08d-441c-9583-164bec06e69a) | ![eberly-14](https://github.com/user-attachments/assets/612e09c4-dc5e-4c72-b516-bb4ea6a6dc07) | ![elgindy-1](https://github.com/user-attachments/assets/57cd1cb3-9a7a-4c75-994f-a0b01ec87daa) |
| **Gray Embroidery** | **Held 1** | **Held 3** |
| ![gray-embroidery](https://github.com/user-attachments/assets/3206f3a4-82a4-4013-a1ff-082484b28e29) | ![held-1](https://github.com/user-attachments/assets/a4d4f651-b5f2-47d2-811c-a31be9bff848) | ![held-3](https://github.com/user-attachments/assets/019d28b8-d5d5-4e19-976f-388e97df95bc) |
| **Held 7a** | **Held 7b** | **Held 7c** |
| ![held-7a](https://github.com/user-attachments/assets/d3cd5fd2-3778-4514-9317-389014e258e2) | ![held-7b](https://github.com/user-attachments/assets/7e7178e7-25c8-4afd-811f-839e92fa6341) | ![held-7c](https://github.com/user-attachments/assets/443af751-801d-4ceb-a96c-2a2447d5cd88) |
| **Held 7d** | **Held 12** | **Mapbox Building** |
| ![held-7d](https://github.com/user-attachments/assets/2a987b0f-2112-4d47-b785-f3fe1b4772e5) | ![held-12](https://github.com/user-attachments/assets/eff74831-a4b1-44fd-bb47-35bf982e9778) | ![mapbox-building](https://github.com/user-attachments/assets/1d036ccf-f2c3-45d5-a5d4-7e5474121f7b) |
| **Mapbox Dude** | **Matisse Alga** | **Matisse Blue** |
| ![mapbox-dude](https://github.com/user-attachments/assets/dbb19574-b05f-4904-b130-c9c532ed9698) | ![matisse-alga](https://github.com/user-attachments/assets/ca86a6bf-6a52-47a4-92b9-cf4eaf45c576) | ![matisse-blue](https://github.com/user-attachments/assets/23a47de5-c4e3-412e-a9ee-9ce0e52b99d7) |
| **Matisse Icarus** | **Matisse Nuit** | **Mei 2** |
| ![matisse-icarus](https://github.com/user-attachments/assets/b233f822-b4b6-4e63-a0da-de5b65674fb3) | ![matisse-nuit](https://github.com/user-attachments/assets/dcb1d0ab-5cdf-40c4-8acb-865cacbf6534) | ![mei-2](https://github.com/user-attachments/assets/a35ab0ea-3109-4346-9972-4bd735609508) |
| **Mei 3** | **Mei 4** | **Mei 5** |
| ![mei-3](https://github.com/user-attachments/assets/ffbef5ca-8e8a-4e3a-b23f-be184022ffd8) | ![mei-4](https://github.com/user-attachments/assets/9b3510c9-6426-4ade-a5ae-30008f87f9a8) | ![mei-5](https://github.com/user-attachments/assets/748d0428-e178-48a9-88d8-45cc6a3e88e1) |
| **Mei 6** | **Meisters 3** | **Misc Discobolus** |
| ![mei-6](https://github.com/user-attachments/assets/69239dba-00ac-4eca-816d-c5bc2ba0c1ca) | ![meisters-3](https://github.com/user-attachments/assets/2f9ff5b5-00ad-4f9b-9bb8-e6b6ec560e57) | ![misc-discobolus](https://github.com/user-attachments/assets/1101debe-688a-4cba-9eeb-c31a257f5831) |
| **Misc Fu** | **Seidel 3** | **Skimage Horse** |
| ![misc-fu](https://github.com/user-attachments/assets/d78c6457-009b-4c9b-8c8c-03d4d2102b08) | ![seidel-3](https://github.com/user-attachments/assets/06060683-bb65-4cce-bcfc-a0b71117db00) | ![skimage-horse](https://github.com/user-attachments/assets/60b664ea-e790-44fe-b8d9-7e06a90f9fd2) |
| **Toussaint 1a** |
| ![toussaint-1a](https://github.com/user-attachments/assets/28e7e91f-ce26-427b-a2d6-00c8623a705d) |

---

## Differences from IPA Dataset

I made a few modifications to make the definitions suitable for this library. I copied the points from the [JSON directory](https://github.com/LingDong-/interesting-polygon-archive/tree/master/json) and asked ChatGPT to format them into the JSON formatted expected by this library.

Polygons with holes are not yet supported by this repo, but the IPA dataset had several with holes. I adopted the practice of taking only the outer boundary from the IPA polygons if they had holes (the first line of points in their JSON files). This means some of them look a little silly. When this repo supports holes, I will circle back and fix them.

I discovered they were upside down because the IPA dataset assumed a the +Y axis was pointing down, while I assume it's pointing up in my visualizer. So, I rotated all of them 180 degrees about the origin and translated them back to the first quadrant of the plot. I realize this makes them face a different direction than the IPA dataset, but I'm okay with that.

---
