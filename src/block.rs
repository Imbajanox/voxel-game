/// Represents different types of blocks in the voxel world
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
    Water,
}

impl BlockType {
    /// Returns true if the block is solid (not air or water)
    pub fn is_solid(&self) -> bool {
        !matches!(self, BlockType::Air | BlockType::Water)
    }

    /// Returns true if the block is transparent
    pub fn is_transparent(&self) -> bool {
        matches!(self, BlockType::Air | BlockType::Water)
    }

    /// Returns the color of the block as RGB
    pub fn color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Grass => [0.2, 0.8, 0.2],
            BlockType::Dirt => [0.6, 0.4, 0.2],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Water => [0.2, 0.4, 0.8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_solidity() {
        assert!(!BlockType::Air.is_solid());
        assert!(BlockType::Grass.is_solid());
        assert!(BlockType::Stone.is_solid());
        assert!(!BlockType::Water.is_solid());
    }

    #[test]
    fn test_block_transparency() {
        assert!(BlockType::Air.is_transparent());
        assert!(!BlockType::Grass.is_transparent());
        assert!(BlockType::Water.is_transparent());
    }
}
