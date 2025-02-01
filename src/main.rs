use export::export_image;
use ray_tracer::{dense::DenseStorage, RayTracer};
use voxel::VoxelGenerator;

pub mod export;
pub mod ray_tracer;
pub mod voxel;

#[tokio::main]
async fn main() {
    // Create voxel data.
    let voxel_generator = VoxelGenerator::new();
    // Create ray tracer.
    let ray_tracer = RayTracer::<DenseStorage>::from_voxels(voxel_generator);
    // Run ray tracer.
    let fb = ray_tracer.render().await;
    // Export image.
    export_image(fb, "./render.png").expect("failed to export image");
}
