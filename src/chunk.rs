use crate::block::BlockType;

/// Size of a chunk in blocks (width, height, depth)
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 64;

/// Represents a chunk of voxel data
pub struct Chunk {
    /// 3D array of blocks [x][y][z]
    blocks: [[[BlockType; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
    /// Position of the chunk in world coordinates
    pub position: (i32, i32),
}

impl Chunk {
    /// Creates a new empty chunk filled with air
    pub fn new(position: (i32, i32)) -> Self {
        Self {
            blocks: [[[BlockType::Air; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
            position,
        }
    }

    /// Gets the block at the specified local coordinates
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<BlockType> {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return None;
        }
        Some(self.blocks[x][y][z])
    }

    /// Sets the block at the specified local coordinates
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) -> bool {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return false;
        }
        self.blocks[x][y][z] = block;
        true
    }

    /// Generates terrain for this chunk using simple height map
    pub fn generate_terrain(&mut self, noise: &noise::Perlin) {
        use noise::NoiseFn;
        
        let (chunk_x, chunk_z) = self.position;
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_x * CHUNK_SIZE as i32 + x as i32;
                let world_z = chunk_z * CHUNK_SIZE as i32 + z as i32;
                
                // Generate height using Perlin noise
                let noise_value = noise.get([
                    world_x as f64 * 0.01,
                    world_z as f64 * 0.01,
                ]);
                
                // Convert noise (-1 to 1) to height (10 to 30)
                let height = ((noise_value + 1.0) * 10.0 + 10.0) as usize;
                let height = height.min(CHUNK_HEIGHT - 1);
                
                // Fill blocks from bottom to height
                for y in 0..=height {
                    let block = if y == height {
                        BlockType::Grass
                    } else if y > height - 3 {
                        BlockType::Dirt
                    } else {
                        BlockType::Stone
                    };
                    self.set_block(x, y, z, block);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new((0, 0));
        assert_eq!(chunk.position, (0, 0));
        assert_eq!(chunk.get_block(0, 0, 0), Some(BlockType::Air));
    }

    #[test]
    fn test_set_and_get_block() {
        let mut chunk = Chunk::new((0, 0));
        assert!(chunk.set_block(5, 10, 7, BlockType::Stone));
        assert_eq!(chunk.get_block(5, 10, 7), Some(BlockType::Stone));
    }

    #[test]
    fn test_out_of_bounds() {
        let chunk = Chunk::new((0, 0));
        assert_eq!(chunk.get_block(CHUNK_SIZE, 0, 0), None);
        assert_eq!(chunk.get_block(0, CHUNK_HEIGHT, 0), None);
    }
}
