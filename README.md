# Sbixel

**Sbixel** is a very simple pixel physics simulator I made to learn Rust.  
It uses [macroquad](https://github.com/not-fl3/macroquad) for drawing — and I was pleasantly surprised by how simple and awesome it is!

**Quick demo (click for youtube link):**

[![Watch the video](https://img.youtube.com/vi/HC_0ff91lWg/hqdefault.jpg)](https://www.youtube.com/watch?v=HC_0ff91lWg)



> ⚠️ This project is very basilar and doesn't have much ambition to go anywhere — it's mainly a learning sandbox.

## How It Works

The simulation uses a "sector" system to reduce unnecessary processing by only simulating active areas of pixels.  
All settings related to simulation and performance can be found in [`src/def.rs`](src/def.rs).

## TODO List (aka: probably won't do soon 😅)

That said, here's the current (loose) roadmap. If anyone wants to contribute, I’d be happy to give my support:

- 🧪 Upgrade water simulation
- 🌊 Improve sand/water interaction
- 🖼️ Render pixels only in **active sectors** instead of rewriting the whole window every frame
- 🧠 Refactor `pixel_already_processed` logic — it's very trivial and causes potential bugs
- 💨 Add gas simulation
- 🧱 Add static objects

## Getting Started

Make sure you have Rust installed. Then, run the project with (for performance go --release):

```bash
cargo run
```

Feel free to explore, break things, and maybe even improve the code 😊

