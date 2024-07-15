# `bevy_heightmap`


[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Katsutoshii/bevy_heightmap#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_heightmap.svg)](https://crates.io/crates/bevy_heightmap)
[![Docs](https://docs.rs/bevy_heightmap/badge.svg)](https://docs.rs/bevy_heightmap/latest/bevy_heightmap/)

Load height map PNGs as meshes in `bevy`.

## Usage

Create a height map from a value function:

```rust
use bevy::prelude::*;
use bevy_heightmap::*;
let heightmap = HeightMap {
  size: UVec2::new(10, 10),
  h: |p: Vec2| ((20. * p.x).sin() + (20. * p.y).sin()) / 2.
};
let mesh: Mesh = heightmap.into();
assert_eq!(mesh.count_vertices(), 4 * 10 * 10);
```

Load a height map as a mesh from an image (requires `.hmp.png` extension):

```rust
use bevy::prelude::*;
use bevy_heightmap::*;
fn setup(asset_server: Res<AssetServer>) {
    let mesh: Handle<Mesh> = asset_server.load("textures/terrain.hmp.png");
}
```

## Examples

```
cargo run --example image
```
