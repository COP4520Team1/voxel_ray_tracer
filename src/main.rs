use std::path::PathBuf;

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
    #[arg(short, long, default_value_t = 50)]
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

    /// Enable octree debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), &'static str> {
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
        backend: storage,
        size,
        position,
        seed,
        out,
        debug,
    } = Cli::parse(); // Parses command-line arguments

    // Print parsed arguments
    println!("Storage Mode: {:?}", storage);
    println!("Scene Size: {}", size);

    let position = match &position {
        Some(pos) if pos.len() == 3 => {
            println!("Scene Position: {:?}", pos);
            IVec3::from_slice(pos)
        }
        Some(_) => return Err("Invalid position format! Use --pos x,y,z"),
        None => (size as i32 - 10).max(10) * IVec3::ONE,
    };

    println!("Seed: {:?}", seed.unwrap_or_else(|| rand::random()));

    let output_path = {
        let path = PathBuf::from(&out);
        if path.is_absolute() {
            path
        } else {
            PathBuf::from("./").join(path)
        }
    };

    println!("Output File: {}", output_path.display());

    let config = Config {
        seed,
        res_width: 7680,
        res_height: 4320,
        camera_pos: position.as_vec3a(),
        size,
        debug,
    };

    let fb = match storage.unwrap_or_default() {
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
    export_image(fb, out).expect("failed to export image");

    Ok(())
}
