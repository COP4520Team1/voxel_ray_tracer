use export::export_image;
use glam::IVec3;
use ray_tracer::{dense::DenseStorage, RayTracer};
use voxel::VoxelGenerator;

pub mod export;
pub mod ray_tracer;
pub mod voxel;

#[tokio::main]
async fn main() {
    // Create voxel data.
    let voxel_generator = VoxelGenerator::new();
    let bounds = (IVec3::new(-10, -10, -10), IVec3::new(10, 10, 10));
    // Create ray tracer.
    let ray_tracer = RayTracer::<DenseStorage>::from_voxels(voxel_generator, bounds);
    // Run ray tracer.
    let fb = ray_tracer.render().await;
    // Export image.
    export_image(fb, "./render.png").expect("failed to export image");
}
