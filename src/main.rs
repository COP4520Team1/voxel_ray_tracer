use clap::{Parser, ValueEnum};
use glam::IVec3;
use std::path::PathBuf;
use voxel_ray_tracer::{
    export::export_image,
    ray_tracer::{dense::DenseStorage, octree::SparseStorage, types::IAabb, RayTracer},
    voxel::VoxelGenerator,
};

/// Define possible storage modes
#[derive(Debug, Clone, ValueEnum)]
enum StorageMode {
    Sparse,
    Dense,
}

/// Command-line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Choose storage mode: --sparse or --dense (required)
    #[arg(short, long, value_enum)]
    storage: StorageMode,

    /// Scene size (optional, default = 50)
    #[arg(long, default_value_t = 50)]
    size: u32,

    /// Scene position (optional, format: x,y,z)
    #[arg(long, value_delimiter = ',')]
    pos: Option<Vec<i32>>,

    /// Seed value (optional, default = random)
    #[arg(long)]
    seed: Option<u32>, // Ensure it’s u32

    /// Output image file path (optional, default = "render.jpg")
    #[arg(long, default_value = "render.jpg")]
    out: String,
}

fn main() {
    let cli = Cli::parse(); // Parses command-line arguments

    // Handle optional seed
    let seed: u32 = cli.seed.unwrap_or_else(|| rand::random::<u32>());

    println!("Storage Mode: {:?}", cli.storage);
    println!("Scene Size: {}", cli.size);

    match &cli.pos {
        Some(pos) if pos.len() == 3 => println!("Scene Position: {:?}", pos),
        Some(_) => println!("Invalid position format! Use --pos x,y,z"),
        None => println!("No position specified."),
    }

    println!("Seed: {:?}", seed);

    // Ensure output path is properly formatted
    let output_path = {
        let path = PathBuf::from(&cli.out);
        if path.is_absolute() {
            path
        } else {
            PathBuf::from("./").join(path)
        }
    };

    println!("Output File: {}", output_path.display());

    let voxel_generator = VoxelGenerator::new_from_seed(seed);
    let bb = IAabb::new(IVec3::ZERO, IVec3::splat(cli.size as i32));

    match cli.storage {
        StorageMode::Sparse => {
            println!("Constructing scene...");
            let ray_tracer = RayTracer::<SparseStorage>::from_voxels(&voxel_generator, bb);
            println!("Running ray tracer...");
            let fb = ray_tracer.render();
            println!("Saving image...");
            export_image(fb, output_path.to_str().unwrap()) // Convert PathBuf to str
                .expect("failed to export image");
        }
        StorageMode::Dense => {
            println!("Constructing scene...");
            let ray_tracer = RayTracer::<DenseStorage>::from_voxels(&voxel_generator, bb);
            println!("Running ray tracer...");
            let fb = ray_tracer.render();
            println!("Saving image...");
            export_image(fb, output_path.to_str().unwrap()) // Convert PathBuf to str
                .expect("failed to export image");
        }
    }
}
