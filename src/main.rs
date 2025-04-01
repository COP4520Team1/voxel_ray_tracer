use voxel_ray_tracer::{
    export::export_image,
    ray_tracer::{octree::SparseStorage, Config, RayTracer},
};

#[cfg(feature = "trace")]
use tracing_subscriber::prelude::*;

fn main() {
    // Setup tracing scaffold.
    #[cfg(feature = "trace")]
    {
        let fmt_layer = tracing_subscriber::fmt::layer(); // writes to stdout
        let tracy_layer = tracing_tracy::TracyLayer::default(); // writes to tracy port
        let registry = tracing_subscriber::registry()
            .with(fmt_layer)
            .with(tracy_layer)
            .init();
    };

    // Create ray tracer.
    println!("Constructing scene...");
    let ray_tracer = RayTracer::<SparseStorage>::new(Config {
        seed: Some(0),
        res_width: 7680,
        res_height: 4320,
        camera_pos: 240.0 * glam::Vec3A::ONE,
        size: 250,
        debug: false,
    });
    // Run ray tracer.
    println!("Running ray tracer...");
    let fb = ray_tracer.render();
    // Export image.
    println!("Saving image...");
    export_image(fb, "./render.png").expect("failed to export image");
}
