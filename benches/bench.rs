use criterion::*;
use glam::IVec3;
use voxel_ray_tracer::{
    ray_tracer::{dense::DenseStorage, octree::SparseStorage, types::IAabb, RayTracer},
    voxel::VoxelGenerator,
};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage-solution");

    let voxel_generator = VoxelGenerator::new();
    let bb = IAabb::new(IVec3::ZERO, 250 * IVec3::ONE);

    let dense_ray_tracer = RayTracer::<DenseStorage>::from_voxels(&voxel_generator, bb);
    let sparse_ray_tracer = RayTracer::<SparseStorage>::from_voxels(&voxel_generator, bb);

    group.bench_function("dense", |b| b.iter(|| black_box(dense_ray_tracer.render())));

    group.bench_function("sparse", |b| {
        b.iter(|| black_box(sparse_ray_tracer.render()))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
