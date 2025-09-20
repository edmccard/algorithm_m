use criterion::{Criterion, criterion_group, criterion_main};

use algorithm_m::choose::{Choose, FirstWins, MRVChooser, NoPreference};
use algorithm_m::links::{INode, INodes, ONode};
use algorithm_m::items::Items;
use algorithm_m::{Count, ODance, Problem};

fn bench_dance(c: &mut Criterion) {
    let items = INode::make_nodes(7, 0);
    let os: Vec<Vec<Count>> = vec![
        vec![2, 4],
        vec![0, 3, 6],
        vec![1, 2, 5],
        vec![0, 3, 5],
        vec![1, 6],
        vec![3, 4, 6],
    ];
    let opts = ONode::make_nodes(7, 0, 6, 16, os);
    let tiebreak: FirstWins<INodes> = FirstWins::new();
    let mut chooser = MRVChooser::new(NoPreference(), tiebreak);
    let mut problem = Problem::new(items, opts);

    c.bench_function("dance", |b| {
        b.iter(|| {
            solve(&mut problem, &mut chooser);
        })
    });
}

fn solve<I: Items, O: ODance, C: Choose<I>>(
    problem: &mut Problem<I, O>,
    chooser: &mut C,
) -> usize {
    let mut i = 0;
    while problem.next_solution(chooser) {
        i += 1;
    }
    i
}

criterion_group!(benches, bench_dance);
criterion_main!(benches);
