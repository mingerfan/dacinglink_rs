mod bench_utils;

use criterion::{criterion_group, criterion_main, measurement::WallTime, Criterion};
use dancinglink::dancinglink_multicover_v1::DlMulti;

fn gen_dl_cases(r: usize, c: usize, solution_rows: usize, times: usize) -> Vec<DlMulti> {
    let mut dl_vec = vec![];
    for _ in 0..times {
        let (matrix, _) = bench_utils::generate_muticover_matrix(r, c, solution_rows);
        let mut dl = DlMulti::new(r, c);
        for (r_in, c_vec) in matrix.iter().enumerate() {
            for (c_in, item) in c_vec.iter().enumerate() {
                if *item == 1 {
                    dl.insert(r_in + 1, c_in + 1)
                }
            }
        }
        dl_vec.push(dl);
    }
    dl_vec
}

fn group_bench(
    g: &mut criterion::BenchmarkGroup<'_, WallTime>,
    r: usize,
    c: usize,
    solution_rows: usize,
) {
    let id = format!("r{}c{}solution_rows{}", r, c, solution_rows);
    let cases = gen_dl_cases(r, c, solution_rows, 1);
    g.bench_function(id, |b| {
        b.iter_batched_ref(
            || cases.clone(),
            |cases| {
                for case in cases {
                    let _ = case.dance();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn benchmark_dl_multi_with_different_rc(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, WallTime> = c.benchmark_group("DL multi with different rc");
    group_bench(&mut group, 10, 10, 5);
    group_bench(&mut group, 20, 20, 5);
    group_bench(&mut group, 30, 30, 5);
    group_bench(&mut group, 40, 40, 5);
    group_bench(&mut group, 50, 50, 5);
    group_bench(&mut group, 60, 60, 5);
    group_bench(&mut group, 100, 100, 5);
    group_bench(&mut group, 200, 200, 5);
    group_bench(&mut group, 300, 300, 5);
    group.finish();
}

criterion_group!(
    benches_multi,
    benchmark_dl_multi_with_different_rc
);
criterion_main!(benches_multi);
