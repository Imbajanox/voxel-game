# Voxel Game Engine Architecture

## Overview

This is a Rust-based voxel game engine built with modern graphics APIs, designed to be similar to Minecraft. The engine uses a chunk-based terrain system with procedural generation.

## Core Components

### 1. Block System (`src/block.rs`)

The block system defines different types of voxels in the world.

```rust
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
    Water,
}
```

**Features:**
- Solidity checking (solid vs. air/water)
- Transparency detection
- Color properties for rendering
- Extensible design for adding new block types

### 2. Chunk System (`src/chunk.rs`)

Chunks are 3D containers of blocks that make up the world.

**Specifications:**
- Size: 16x64x16 blocks
- 3D array storage: `[x][y][z]`
- Procedural terrain generation using Perlin noise
- Efficient block access and modification

**Key Methods:**
- `new(position)`: Create a new chunk
- `get_block(x, y, z)`: Retrieve block at position
- `set_block(x, y, z, block)`: Set block at position
- `generate_terrain(noise)`: Generate terrain using Perlin noise

### 3. Camera System (`src/camera.rs`)

First-person camera with smooth movement and rotation.

**Features:**
- Position tracking in 3D space
- Yaw/pitch rotation (prevents camera flipping)
- Movement in 6 directions (forward, back, left, right, up, down)
- View and projection matrix generation
- Configurable FOV, aspect ratio, near/far planes

**Movement:**
- WASD: Horizontal movement
- Space/Shift: Vertical movement
- Arrow keys: Camera rotation

### 4. Rendering System (`src/renderer.rs`)

Modern GPU-based rendering using wgpu (WebGPU).

**Components:**
- Vertex and index buffers for geometry
- Camera uniform buffer for transformations
- Render pipeline with custom shaders
- Surface configuration and management

**Rendering Pipeline:**
1. Create vertex data with positions and colors
2. Update camera uniform buffer
3. Submit draw calls to GPU
4. Present frame to screen

### 5. Shader System (`src/shader.wgsl`)

WGSL (WebGPU Shading Language) shaders for rendering.

**Vertex Shader:**
- Transforms vertices using camera view-projection matrix
- Passes color data to fragment shader

**Fragment Shader:**
- Applies per-pixel colors
- Simple solid color rendering

## Data Flow

```
User Input → Camera Movement → Update Camera Uniform → Render Pipeline → Display
     ↓
Chunk Generation → Block Data → Vertex Generation → GPU Rendering
```

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Graphics API | wgpu 0.19 | Modern cross-platform GPU rendering |
| Windowing | winit 0.29 | Cross-platform window management |
| Math | glam 0.25 | 3D mathematics and transformations |
| Noise | noise 0.8 | Procedural terrain generation |
| Async | pollster 0.3 | Async runtime utilities |

## Performance Considerations

### Current Implementation
- Single chunk rendering (demonstration)
- Simple cube geometry
- No texture mapping
- Basic color-based rendering

### Future Optimizations
- Multi-chunk rendering with view frustum culling
- Face culling (don't render hidden block faces)
- Greedy meshing to reduce vertex count
- Level of detail (LOD) system
- Texture atlases for block textures
- Occlusion culling

## Memory Layout

```
Chunk (16x64x16 blocks)
├── Block array [x][y][z]
│   └── Each block: BlockType (1 byte enum)
├── Position (i32, i32)
└── Total: ~16KB per chunk
```

## Coordinate System

- **X-axis**: West (-) to East (+)
- **Y-axis**: Down (-) to Up (+)
- **Z-axis**: North (-) to South (+)
- **Chunk coordinates**: Multiples of CHUNK_SIZE (16)
- **Block coordinates**: 0 to CHUNK_SIZE-1 within chunk

## Testing

The engine includes comprehensive unit tests:

```bash
cargo test
```

**Test Coverage:**
- Block property verification
- Chunk creation and manipulation
- Camera movement and rotation
- Coordinate boundary checking

## Building and Running

### Development Build
```bash
cargo build
cargo run
```

### Release Build (Optimized)
```bash
cargo build --release
cargo run --release
```

### Run Example
```bash
cargo run --example terrain_generation
```

## Examples

### Terrain Generation Example
Demonstrates:
- Chunk creation
- Procedural terrain using Perlin noise
- Block statistics
- Block property queries

Run with: `cargo run --example terrain_generation`

## Future Enhancements

### Short Term
- [ ] Multiple chunk rendering
- [ ] Block placement/destruction
- [ ] Texture mapping
- [ ] Better camera controls (mouse look)

### Medium Term
- [ ] Advanced terrain generation (caves, trees, biomes)
- [ ] Lighting system (ambient + directional)
- [ ] Physics (gravity, collision detection)
- [ ] Save/load world data

### Long Term
- [ ] Multiplayer networking
- [ ] Advanced rendering (shadows, ambient occlusion)
- [ ] Mod support
- [ ] Performance profiling and optimization

## Code Style

- Rust 2021 edition
- Clippy linting enabled
- Comprehensive documentation
- Unit tests for core functionality
- Idiomatic Rust patterns

## License

MIT License - See LICENSE file for details
