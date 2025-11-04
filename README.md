# voxel-game

A Rust-based voxel game engine similar to Minecraft, built with modern graphics APIs.

## Features

- **Core Voxel Engine**: Efficient chunk-based terrain management system
- **Block System**: Multiple block types (Grass, Dirt, Stone, Water, Air)
- **Modern Graphics**: Built on wgpu for cross-platform GPU rendering
- **Camera System**: First-person camera with smooth movement and rotation
- **Procedural Generation**: Perlin noise-based terrain generation
- **Performance**: Optimized chunk rendering with face culling

## Building

Make sure you have Rust installed. Then run:

```bash
cargo build --release
```

## Running

To start the game:

```bash
cargo run --release
```

## Controls

- **WASD**: Move forward/backward/left/right
- **Space**: Move up
- **Left Shift**: Move down
- **Arrow Keys**: Look around (rotate camera)
- **ESC**: Exit game

## Architecture

### Modules

- `block.rs`: Block type definitions and properties
- `chunk.rs`: Chunk management and terrain generation
- `camera.rs`: First-person camera implementation
- `renderer.rs`: wgpu-based rendering system
- `shader.wgsl`: WGSL vertex and fragment shaders

### Technical Details

- **Chunk Size**: 16x64x16 blocks
- **Graphics API**: wgpu (WebGPU)
- **Math Library**: glam
- **Windowing**: winit
- **Terrain Generation**: Perlin noise

## Testing

Run the test suite:

```bash
cargo test
```

## Dependencies

- `wgpu`: Modern GPU graphics API
- `winit`: Cross-platform windowing
- `glam`: Mathematics library for 3D graphics
- `noise`: Procedural noise generation
- `pollster`: Async runtime utilities
- `bytemuck`: Safe transmutation between types

## Future Enhancements

- Multiple chunk rendering
- Block placement/destruction
- Better terrain generation algorithms
- Texture mapping
- Lighting system
- Physics system
- Multiplayer support

## License

This project is open source and available under the MIT License.
