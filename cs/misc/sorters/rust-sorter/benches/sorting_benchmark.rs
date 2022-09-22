use criterion::{criterion_group, criterion_main, Criterion};
use sort::Sorter;

fn benchmark_sorts(c: &mut Criterion) {
    let length = 10_000;

    macro_rules! bench_sorter_method {
        ( $meth:ident ) => {
            c.bench_function(stringify!($meth), |b| {
                b.iter(|| Sorter::new(length).$meth())
            });
        };
    }

    //bench_sorter_method!(bogo_sort);
    bench_sorter_method!(bubble_sort);
    bench_sorter_method!(insertion_sort);
    bench_sorter_method!(merge_sort);
    bench_sorter_method!(stalin_sort);
    bench_sorter_method!(std_sort);
}

criterion_group!(benches, benchmark_sorts);
criterion_main!(benches);
