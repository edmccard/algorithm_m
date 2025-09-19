use criterion::{Criterion, criterion_group, criterion_main};

use algorithm_m::links::{INode, ONode};
use algorithm_m::{IDance, Mrv, ODance, Problem};

fn bench_dance(c: &mut Criterion) {
    let items = INode::make_nodes(7, 0);
    let os: Vec<Vec<usize>> = vec![
        vec![2, 4],
        vec![0, 3, 6],
        vec![1, 2, 5],
        vec![0, 3, 5],
        vec![1, 6],
        vec![3, 4, 6],
    ];
    let opts = ONode::make_nodes(7, 6, 16, os);
    let mut chooser = Mrv {};
    let mut problem = Problem::new(items, opts);

    c.bench_function("dance", |b| {
        b.iter(|| {
            solve(&mut problem, &mut chooser);
        })
    });
}

fn solve<I: IDance, O: ODance>(
    problem: &mut Problem<I, O>,
    chooser: &mut Mrv,
) -> usize {
    let mut i = 0;
    while problem.next_solution(chooser) {
        i += 1;
    }
    i
}

criterion_group!(benches, bench_dance);
criterion_main!(benches);
