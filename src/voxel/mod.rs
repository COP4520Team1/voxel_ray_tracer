use glam::{IVec3, U8Vec3};
use noise::{NoiseFn, Perlin};
use rand::Rng;

/// Data associated with a single voxel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Voxel {
    pub color: U8Vec3,
}

/// An iterator that produces voxels with z coordinate calculated by Perlin noise function mapped over x and y coordinates, and voxel color is mapped from max voxel height at its x and y coordinate
#[derive(Clone)]
pub struct VoxelGenerator {
    perlin: Perlin,
}

/// Max height of the voxel
const HEIGHT: i32 = 100;

/// Roughness for the Perlin Noise Function. Lower values represent smoother terrain, higher values represent rougher terrain
const ROUGHNESS: f64 = 1.0;

/// Scales the Roughness to the max height of the voxel (to keep the roughness consistent across different max heights)
const SCALE: f64 = ROUGHNESS / HEIGHT as f64;

// Colors for use in height_to_color function
const WATER_BLUE: U8Vec3 = U8Vec3::new(0, 80, 200);
const GRASS_GREEN: U8Vec3 = U8Vec3::new(50, 170, 50);
const MOUNTAIN_GRAY: U8Vec3 = U8Vec3::new(130, 130, 130);
const SNOW_WHITE: U8Vec3 = U8Vec3::new(240, 240, 255);

impl VoxelGenerator {
    /// Create a new voxel generator with random seed.
    pub fn new() -> Self {
        let seed: u32 = rand::rng().random::<u32>();
        let perlin = Perlin::new(seed);
        Self { perlin }
    }

    /// Creates a new voxel generator with set seed (for testing purposes)
    pub fn new_from_seed(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        Self { perlin }
    }

    /// Lookup a voxel value at some position.
    pub fn lookup(&self, pos: IVec3) -> Option<Voxel> {
        // Calculate the Perlin noise value at (x, y)
        let nx = pos.x as f64 * SCALE;
        let ny = pos.y as f64 * SCALE;
        let noise_value = self.perlin.get([nx, ny]);

        // Calculate the terrain height based on the noise value
        let terrain_z = ((noise_value + 1.0) / 2.0 * HEIGHT as f64) as i32;

        // Check if the voxel exists at the requested position (z should be <= terrain_z)
        if pos.z >= 0 && pos.z <= terrain_z {
            Some(Voxel {
                color: Self::height_to_color(terrain_z),
            })
        } else {
            None
        }
    }

    fn height_to_color(z: i32) -> U8Vec3 {
        let normalized = z as f32 / HEIGHT as f32;

        if normalized < 0.3 {
            WATER_BLUE
        } else if normalized < 0.6 {
            GRASS_GREEN
        } else if normalized < 0.8 {
            MOUNTAIN_GRAY
        } else {
            SNOW_WHITE
        }
    }
}

#[cfg(test)]
mod tests {
    use noise::Seedable;

    use super::*;

    const TEST_SEED: u32 = 12345;

    #[test]
    fn test_voxel_generator_lookup_with_seed() {
        let voxel_generator = VoxelGenerator::new_from_seed(TEST_SEED);

        // Define coordinates to test
        let x = 0;
        let y = 0;

        // Calculate the Perlin noise value at (5, 5)
        let perlin = Perlin::new(TEST_SEED);
        let nx = x as f64 * SCALE;
        let ny = y as f64 * SCALE;
        let noise_value = perlin.get([nx, ny]);

        // Calculate the terrain height based on the noise value
        let terrain_z = ((noise_value + 1.0) / 2.0 * HEIGHT as f64) as i32;

        // Check if the voxel exists at the calculated height
        let voxel_correct_height = voxel_generator.lookup(IVec3::new(x, y, terrain_z));
        assert!(
            voxel_correct_height.is_some(),
            "Voxel does not exist at correct height"
        );

        // Check if the voxel above the calculated height does not exist
        let voxel_above = voxel_generator.lookup(IVec3::new(x, y, terrain_z + 1));
        assert!(
            voxel_above.is_none(),
            "Voxel exists above the calculated height"
        );

        // Ensure voxel exists below or at calculated height
        let voxel_below = voxel_generator.lookup(IVec3::new(x, y, terrain_z - 1));
        assert!(
            voxel_below.is_some() || terrain_z == 0,
            "Voxel is missing below or at the calculated height"
        );
    }

    #[test]
    fn test_seed_generation() {
        let voxel_gen_1 = VoxelGenerator::new();
        let voxel_gen_2 = VoxelGenerator::new();

        assert_ne!(voxel_gen_1.perlin.seed(), voxel_gen_2.perlin.seed(), "Either the seeds were randomly generated to be the same (test case by rerunning test) or seed generation is not working properly");
    }

    #[test]
    fn test_voxel_color_mapping() {
        let low_voxel = VoxelGenerator::height_to_color(2);
        let mid_voxel = VoxelGenerator::height_to_color(HEIGHT / 2);
        let high_voxel = VoxelGenerator::height_to_color(HEIGHT - 1);

        assert_eq!(low_voxel, WATER_BLUE, "Low altitude should be blue (water)");
        assert_eq!(
            mid_voxel, GRASS_GREEN,
            "Mid altitude should be green (grass)"
        );
        assert_eq!(
            high_voxel, SNOW_WHITE,
            "High altitude should be white (snow)"
        );
    }
}
