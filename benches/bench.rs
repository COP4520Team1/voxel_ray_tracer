use criterion::*;
use glam::Vec3A;
use voxel_ray_tracer::ray_tracer::{dense::DenseStorage, octree::SparseStorage, Config, RayTracer};

fn bench_1080p(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage-solution-1080p");

    {
        let config = Config {
            seed: None,
            res_width: 1920,
            res_height: 1080,
            size: 50,
            camera_pos: 40.0 * Vec3A::ONE,
            debug: false,
        };

        group.bench_function("dense-50x", |b| {
            b.iter_batched(
                || RayTracer::<DenseStorage>::new(config),
                |tracer| black_box(tracer.render()),
                BatchSize::PerIteration,
            );
        });

        group.bench_function("sparse-50x", |b| {
            b.iter_batched(
                || RayTracer::<SparseStorage>::new(config),
                |tracer| black_box(tracer.render()),
                BatchSize::PerIteration,
            );
        });
    }

    {
        let config = Config {
            seed: None,
            res_width: 1920,
            res_height: 1080,
            size: 100,
            camera_pos: 90.0 * Vec3A::ONE,
            debug: false,
        };

        group.bench_function("dense-100x", |b| {
            b.iter_batched(
                || RayTracer::<DenseStorage>::new(config),
                |tracer| black_box(tracer.render()),
                BatchSize::PerIteration,
            );
        });

        group.bench_function("sparse-100x", |b| {
            b.iter_batched(
                || RayTracer::<SparseStorage>::new(config),
                |tracer| black_box(tracer.render()),
                BatchSize::PerIteration,
            );
        });
    }

    {
        let config = Config {
            seed: None,
            res_width: 1920,
            res_height: 1080,
            size: 250,
            camera_pos: 240.0 * Vec3A::ONE,
            debug: false,
        };

        group.bench_function("dense-250x", |b| {
            b.iter_batched(
                || RayTracer::<DenseStorage>::new(config),
                |tracer| black_box(tracer.render()),
                BatchSize::PerIteration,
            );
        });

        group.bench_function("sparse-250x", |b| {
            b.iter_batched(
                || RayTracer::<SparseStorage>::new(config),
                |tracer| black_box(tracer.render()),
                BatchSize::PerIteration,
            );
        });
    }
}

fn bench_4k(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage-solution-4k");

    {
        let config = Config {
            seed: Some(0),
            res_width: 7680,
            res_height: 4320,
            size: 50,
            camera_pos: 40.0 * Vec3A::ONE,
            debug: false,
        };

        let dense_ray_tracer = RayTracer::<DenseStorage>::new(config);
        let sparse_ray_tracer = RayTracer::<SparseStorage>::new(config);

        group.bench_function("dense-50x", |b| {
            b.iter(|| black_box(dense_ray_tracer.render()))
        });

        group.bench_function("sparse-50x", |b| {
            b.iter(|| black_box(sparse_ray_tracer.render()))
        });
    }

    {
        let config = Config {
            seed: Some(0),
            res_width: 7680,
            res_height: 4320,
            size: 100,
            camera_pos: 90.0 * Vec3A::ONE,
            debug: false,
        };

        let dense_ray_tracer = RayTracer::<DenseStorage>::new(config);
        let sparse_ray_tracer = RayTracer::<SparseStorage>::new(config);

        group.bench_function("dense-100x", |b| {
            b.iter(|| black_box(dense_ray_tracer.render()))
        });

        group.bench_function("sparse-100x", |b| {
            b.iter(|| black_box(sparse_ray_tracer.render()))
        });
    }

    {
        let config = Config {
            seed: Some(0),
            res_width: 7680,
            res_height: 4320,
            size: 250,
            camera_pos: 240.0 * Vec3A::ONE,
            debug: false,
        };

        let dense_ray_tracer = RayTracer::<DenseStorage>::new(config);
        let sparse_ray_tracer = RayTracer::<SparseStorage>::new(config);

        group.bench_function("dense-250x", |b| {
            b.iter(|| black_box(dense_ray_tracer.render()))
        });

        group.bench_function("sparse-250x", |b| {
            b.iter(|| black_box(sparse_ray_tracer.render()))
        });
    }
}

criterion_group!(benches, bench_1080p, bench_4k);
criterion_main!(benches);
