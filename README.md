# ABOUND Starter Repo - Rust

This is a starter repository for an ABOUND algorithm written in [Rust](https://www.rust-lang.org/).

* Unsure on what ABOUND is? Check out https://abound.art
* Looking for another language? Check out our other options for starter repos [here](https://abound.art/artists)
* New to Rust? Get started [here](https://www.rust-lang.org/learn).

## What is in this repo

This repo includes all of the scaffolding to build an algorithm for ABOUND. That means:

1. Reading in the JSON configuration for a run
2. Generating art (using a [Lorenz attractor](https://en.wikipedia.org/wiki/Lorenz_system) as an example)
3. Writing the output to a file

In short, this repo does everything except implement your art algorithm, which
will generally look like this:

```rust
use image::{ImageBuffer, Rgb, RgbImage};
use serde::Deserialize;

// Config is all the parameters your algorithm takes as input.
#[derive(Deserialize)]
pub struct Config {
  seed: u64,
}

pub fn run(c: Config) -> RgbImage {
  // Your code here which generates the image from the config.
}
```

## Run locally + Testing your code

```bash
export ABOUND_CONFIG_PATH=example_input.json
export ABOUND_OUTPUT_PATH=output.png
cargo run
```

Will generate a piece of art at `output.png` that looks like this:

![An example output of the Lorenz attractor algorithm, a blue and green spiral](/example_output.png)

To start implementing your algorithm, replace the `Config` struct and `run`
function in [`src/algo.rs`](/src/algo.rs) with your own. It's also worth noting
that the example algorithm produces raster images, meaning they're made of
pixels and are output as [PNG](https://en.wikipedia.org/wiki/PNG) files, but
you can also write algorithms that produce vector images, meaning they're made
of geometric shapes and are output as
[SVG](https://en.wikipedia.org/wiki/SVG) files.

### As a Docker container

To test the algorithm in a Docker container, run:

```bash
./scripts/build_image.sh
./scripts/run_image.sh <config path> <output path>
```

Where `<config path>` defaults to `example_input.json` and `<output path>`
defaults to `output.png`.

## Packaging for Deployment

Algorithms are uploaded to ABOUND as Docker containers. The example algo can be
built by running:

```bash
./scripts/build_image.sh
```

Which will build the example image and tag it `abound-starter-rust`.

## Deploying on ABOUND 

Head to https://abound.art/artists for the most recent instructions on how to upload
your algorithm once it is written. Make sure to read through the constraints carefully
to make sure that your algorithm conforms to them prior to submission.

Once you're ready to upload, tag and push the image with:

```bash
docker tag <local tag> <image name given by ABOUND>
docker push <image name given by ABOUND>
```

If you haven't changed the example configuration in this repo, the `<local
tag>` defaults to `abound-starter-rust`.

The `docker push` command will fail if you aren't already authenticated with
your ABOUND credentials.
