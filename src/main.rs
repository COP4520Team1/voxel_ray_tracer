use std::path::absolute;

use clap::{Parser, ValueEnum};
use glam::IVec3;

use voxel_ray_tracer::{
    export::export_image,
    ray_tracer::{dense::DenseStorage, octree::SparseStorage, Config, RayTracer},
};

#[cfg(feature = "trace")]
use tracing_subscriber::prelude::*;

/// Define possible storage modes
#[derive(Debug, Clone, ValueEnum, Default)]
enum StorageMode {
    #[default]
    Sparse,
    Dense,
}

/// Command-line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Storage backend
    #[arg(short, long, value_enum)]
    backend: Option<StorageMode>,

    /// Scene size
    #[arg(short, long, default_value_t = 200)]
    size: u32,

    /// Scene position (x,y,z) e.g. 25,25,25
    #[arg(short, long, value_delimiter = ',')]
    position: Option<Vec<i32>>,

    /// Terrain seed value
    #[arg(short = 'r', long)]
    seed: Option<u32>,

    /// Image output path
    #[arg(short, long, default_value = "render.png")]
    out: String,

    /// Image resolution width
    #[arg(short, long, default_value_t = 7680)]
    width: usize,

    /// Image resolution height
    #[arg(short, long, default_value_t = 4320)]
    height: usize,

    /// Enable octree debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let Cli {
        backend,
        size,
        position,
        seed,
        out,
        width,
        height,
        debug,
    } = Cli::parse(); // Parses command-line arguments

    // Print parsed arguments

    let backend = backend.unwrap_or_default();

    println!("Storage Backend: {backend:?}");
    println!("Scene Size: {size}");

    let position = match &position {
        Some(pos) if pos.len() == 3 => {
            println!("Scene Position: {:?}", pos);
            IVec3::from_slice(pos)
        }
        Some(_) => return Err("Invalid position format! Use -p x,y,z".into()),
        None => size as i32 * IVec3::ONE,
    };

    println!("Position: {position}");

    println!("Seed: {seed:?}");

    let output_path = absolute(out)?;

    println!("Output File: {}", output_path.display());
    println!("Resolution: {width}x{height}");

    let config = Config {
        seed,
        res_width: width,
        res_height: height,
        camera_pos: position.as_vec3a(),
        size,
        debug,
    };

    let fb = match backend {
        StorageMode::Sparse => {
            // Create ray tracer.
            println!("Constructing scene...");
            let ray_tracer = RayTracer::<SparseStorage>::new(config);
            // Run ray tracer.
            println!("Running ray tracer...");
            ray_tracer.render()
        }
        StorageMode::Dense => {
            // Create ray tracer.
            println!("Constructing scene...");
            let ray_tracer = RayTracer::<DenseStorage>::new(config);
            // Run ray tracer.
            println!("Running ray tracer...");
            ray_tracer.render()
        }
    };

    // Export image.
    println!("Saving image...");
    export_image(fb, output_path).expect("failed to export image");

    Ok(())
}
