use glam::IVec3;
use voxel_ray_tracer::{
    export::export_image,
    ray_tracer::{dense::DenseStorage, types::IAabb, RayTracer},
    voxel::VoxelGenerator,
};

fn main() {
    // Create voxel data.
<<<<<<< Updated upstream
    let voxel_generator = VoxelGenerator::new();
    let bb = IAabb::new(IVec3::ZERO, 1000 * IVec3::ONE);
=======
    let voxel_generator = VoxelGenerator::new_from_seed(0);
    let bb = IAabb::new(IVec3::ZERO, 100 * IVec3::ONE);
>>>>>>> Stashed changes
    // Create ray tracer.
    let ray_tracer = RayTracer::<DenseStorage>::from_voxels(&voxel_generator, bb);
    // Run ray tracer.
    let fb = ray_tracer.render();
    // Export image.
    export_image(fb, "./render.png").expect("failed to export image");
}
