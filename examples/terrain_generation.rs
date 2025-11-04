/// Example demonstrating the voxel game engine's core features
/// 
/// This example shows:
/// - Creating a chunk with procedural terrain
/// - Block type management
/// - Basic terrain generation

use voxel_game::{Chunk, BlockType, CHUNK_SIZE, CHUNK_HEIGHT};
use noise::Perlin;

fn main() {
    println!("Voxel Game Engine Example");
    println!("==========================\n");

    // Create a new chunk at world position (0, 0)
    let mut chunk = Chunk::new((0, 0));
    println!("Created chunk at position (0, 0)");
    println!("Chunk dimensions: {}x{}x{} blocks\n", CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE);

    // Generate terrain using Perlin noise
    let perlin = Perlin::new(42);
    chunk.generate_terrain(&perlin);
    println!("Generated terrain using Perlin noise");

    // Display some statistics
    let mut block_counts = std::collections::HashMap::new();
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                if let Some(block) = chunk.get_block(x, y, z) {
                    *block_counts.entry(block).or_insert(0) += 1;
                }
            }
        }
    }

    println!("\nBlock statistics:");
    println!("-----------------");
    for (block_type, count) in block_counts.iter() {
        let percentage = (*count as f32 / (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as f32) * 100.0;
        println!("{:?}: {} blocks ({:.1}%)", block_type, count, percentage);
    }

    // Display block properties
    println!("\nBlock properties:");
    println!("-----------------");
    for block_type in [
        BlockType::Air,
        BlockType::Grass,
        BlockType::Dirt,
        BlockType::Stone,
        BlockType::Water,
    ] {
        println!(
            "{:?}: Solid={}, Transparent={}, Color={:?}",
            block_type,
            block_type.is_solid(),
            block_type.is_transparent(),
            block_type.color()
        );
    }

    // Sample some blocks from the chunk
    println!("\nSample blocks from the chunk:");
    println!("------------------------------");
    for y in (0..CHUNK_HEIGHT).step_by(10) {
        if let Some(block) = chunk.get_block(8, y, 8) {
            println!("Position (8, {}, 8): {:?}", y, block);
        }
    }

    println!("\nExample complete! Run 'cargo run' to start the interactive 3D viewer.");
}
