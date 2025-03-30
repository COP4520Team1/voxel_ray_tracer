use glam::IVec3;
use voxel_ray_tracer::{
    export::export_image,
    ray_tracer::{dense::DenseStorage, octree::SparseStorage, types::IAabb, RayTracer},
    voxel::VoxelGenerator,
};

fn main() {
    // Create voxel data.
    let voxel_generator = VoxelGenerator::new_from_seed(0);
    let bb = IAabb::new(IVec3::ZERO, 250 * IVec3::ONE);
    // Create ray tracer.
    println!("Constructing scene...");
    let ray_tracer = RayTracer::<SparseStorage>::from_voxels(&voxel_generator, bb);
    // Run ray tracer.
    println!("Running ray tracer...");
    let fb = ray_tracer.render();
    // Export image.
    println!("Saving image...");
    export_image(fb, "./render.png").expect("failed to export image");
}
