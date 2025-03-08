use glam::IVec3;
use voxel_ray_tracer::{
    export::export_image,
    ray_tracer::{dense::DenseStorage, types::IAabb, RayTracer},
    voxel::VoxelGenerator,
};

fn main() {
    // Create voxel data.
    let voxel_generator = VoxelGenerator::new();
    let bb = IAabb::new(IVec3::ZERO, 1000 * IVec3::ONE);
    // Create ray tracer.
    let ray_tracer = RayTracer::<DenseStorage>::from_voxels(&voxel_generator, bb);
    // Run ray tracer.
    let fb = ray_tracer.render();
    // Export image.
    export_image(fb, "./render.png").expect("failed to export image");
}
