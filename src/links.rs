use crate::{IDance, ODance, OptSpec};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INode {
    left: usize,
    right: usize,
}

impl INode {
    pub fn make_nodes(primary: usize, secondary: usize) -> INodes {
        let mut inodes = INodes {
            nodes: vec![Default::default(); primary + secondary + 2],
            primary,
            secondary,
        };
        inodes.init_links();
        inodes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct INodes {
    nodes: Vec<INode>,
    primary: usize,
    secondary: usize,
}

impl INodes {
    fn get_node(&mut self, i: usize) -> &mut INode {
	if cfg!(feature = "unsafe-fast-index") {
            unsafe { self.nodes.get_unchecked_mut(i) }
	} else {
	    &mut self.nodes[i]
	}
    }
}

impl IDance for INodes {
    #[inline(always)]
    fn primary(&self) -> usize {
        self.primary
    }

    #[inline(always)]
    fn secondary(&self) -> usize {
        self.secondary
    }

    #[inline(always)]
    fn llink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).left
    }
    #[inline(always)]
    fn rlink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).right
    }

    #[inline(always)]
    fn bound(&mut self, _i: usize) -> isize {
        0
    }

    #[inline(always)]
    fn dec_bound(&mut self, _i: usize) -> isize {
        0
    }

    #[inline(always)]
    fn inc_bound(&mut self, _i: usize) -> isize {
        1
    }

    #[inline(always)]
    fn slack(&mut self, _i: usize) -> isize {
        0
    }

    #[inline(always)]
    fn branch_factor(&mut self, _i: usize) -> isize {
        1
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INodeM {
    left: usize,
    right: usize,
    slack: isize,
    bound: isize,
}

impl INodeM {
    pub fn make_nodes(
        primary: usize,
        secondary: usize,
        ms: impl IntoIterator<Item = (isize, isize)>,
    ) -> INodesM {
        let n = primary + secondary;
        let mut inodes = INodesM {
            nodes: vec![Default::default(); n + 2],
            primary,
            secondary,
        };
        for (i, (u, v)) in ms.into_iter().enumerate() {
            inodes.nodes[i + 1].bound = v;
            inodes.nodes[i + 1].slack = v - u;
        }
        inodes.init_links();
        inodes
    }
}

#[derive(Debug)]
pub struct INodesM {
    nodes: Vec<INodeM>,
    primary: usize,
    secondary: usize,
}

impl INodesM {
    fn get_node(&mut self, i: usize) -> &mut INodeM {
	if cfg!(feature = "unsafe-fast-index") {
            unsafe { self.nodes.get_unchecked_mut(i) }
	} else {
	    &mut self.nodes[i]
	}
    }
}

impl IDance for INodesM {
    fn primary(&self) -> usize {
        self.primary
    }

    fn secondary(&self) -> usize {
        self.secondary
    }

    fn llink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).left
    }

    fn rlink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).right
    }

    fn bound(&mut self, i: usize) -> isize {
        self.get_node(i).bound
    }

    fn dec_bound(&mut self, i: usize) -> isize {
        let node = self.get_node(i);
        node.bound -= 1;
        node.bound
    }

    fn inc_bound(&mut self, i: usize) -> isize {
        let node = self.get_node(i);
        node.bound += 1;
        node.bound
    }

    fn slack(&mut self, i: usize) -> isize {
        self.get_node(i).slack
    }

    fn branch_factor(&mut self, i: usize) -> isize {
        let node = self.get_node(i);
        node.bound.saturating_sub(node.slack)
    }
}

impl OptSpec for usize {
    fn get_item(&self) -> usize {
        *self
    }
    fn get_color(&self) -> isize {
        0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ONode {
    hdr_info: isize,
    up: usize,
    down: usize,
}

impl ONode {
    pub fn make_nodes(
        n: usize,
        m: usize,
        l: usize,
        opt_spec: impl IntoIterator<Item = impl IntoIterator<Item = usize>>,
    ) -> ONodes {
        let mut nodes =
            ONodes { nodes: vec![Default::default(); l + m + n + 2] };
        nodes.init_links(n, opt_spec);
        nodes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ONodes {
    nodes: Vec<ONode>,
}

impl ONodes {
    fn get_node(&mut self, i: usize) -> &mut ONode {
        unsafe { self.nodes.get_unchecked_mut(i) }
    }
}

impl ODance for ONodes {
    type Spec = usize;

    #[inline(always)]
    fn olen(&mut self, i: usize) -> &mut isize {
        &mut self.get_node(i).hdr_info
    }

    #[inline(always)]
    fn top(&mut self, i: usize) -> &mut isize {
        &mut self.get_node(i).hdr_info
    }

    #[inline(always)]
    fn ulink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).up
    }

    #[inline(always)]
    fn dlink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).down
    }

    #[inline(always)]
    fn get_color(&mut self, _i: usize) -> isize {
        0
    }

    #[inline(always)]
    fn set_color(&mut self, _i: usize, _c: isize) {}
}

impl OptSpec for (usize, isize) {
    fn get_item(&self) -> usize {
        self.0
    }
    fn get_color(&self) -> isize {
        self.1
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ONodeC {
    hdr_info: isize,
    up: usize,
    down: usize,
    color: isize,
}

impl ONodeC {
    pub fn make_nodes(
        n: usize,
        m: usize,
        l: usize,
        opt_spec: impl IntoIterator<Item = impl IntoIterator<Item = (usize, isize)>>,
    ) -> ONodesC {
        let mut nodes =
            ONodesC { nodes: vec![Default::default(); l + m + n + 2] };
        nodes.init_links(n, opt_spec);
        nodes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ONodesC {
    nodes: Vec<ONodeC>,
}

impl ONodesC {
    fn get_node(&mut self, i: usize) -> &mut ONodeC {
        unsafe { self.nodes.get_unchecked_mut(i) }
    }
}

impl ODance for ONodesC {
    type Spec = (usize, isize);

    fn olen(&mut self, i: usize) -> &mut isize {
        &mut self.get_node(i).hdr_info
    }
    fn top(&mut self, i: usize) -> &mut isize {
        &mut self.get_node(i).hdr_info
    }
    fn ulink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).up
    }
    fn dlink(&mut self, i: usize) -> &mut usize {
        &mut self.get_node(i).down
    }

    fn get_color(&mut self, i: usize) -> isize {
        self.get_node(i).color
    }
    fn set_color(&mut self, i: usize, c: isize) {
        self.get_node(i).color = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    // TAoCP 4B pp. 66-68
    fn test_4b_table_1() {
        let items = INodes {
            nodes: vec![
                INode { left: 7, right: 1 },
                INode { left: 0, right: 2 },
                INode { left: 1, right: 3 },
                INode { left: 2, right: 4 },
                INode { left: 3, right: 5 },
                INode { left: 4, right: 6 },
                INode { left: 5, right: 7 },
                INode { left: 6, right: 0 },
            ],
            primary: 7,
            secondary: 0,
        };
        let opts = ONodes {
            nodes: vec![
                ONode { hdr_info: 0, up: 0, down: 0 },
                ONode { hdr_info: 2, up: 20, down: 12 },
                ONode { hdr_info: 2, up: 24, down: 16 },
                ONode { hdr_info: 2, up: 17, down: 9 },
                ONode { hdr_info: 3, up: 27, down: 13 },
                ONode { hdr_info: 2, up: 28, down: 10 },
                ONode { hdr_info: 2, up: 22, down: 18 },
                ONode { hdr_info: 3, up: 29, down: 14 },
                ONode { hdr_info: 0, up: 0, down: 10 },
                ONode { hdr_info: 3, up: 3, down: 17 },
                ONode { hdr_info: 5, up: 5, down: 28 },
                ONode { hdr_info: -1, up: 9, down: 14 },
                ONode { hdr_info: 1, up: 1, down: 20 },
                ONode { hdr_info: 4, up: 4, down: 21 },
                ONode { hdr_info: 7, up: 7, down: 25 },
                ONode { hdr_info: -2, up: 12, down: 18 },
                ONode { hdr_info: 2, up: 2, down: 24 },
                ONode { hdr_info: 3, up: 9, down: 3 },
                ONode { hdr_info: 6, up: 6, down: 22 },
                ONode { hdr_info: -3, up: 16, down: 22 },
                ONode { hdr_info: 1, up: 12, down: 1 },
                ONode { hdr_info: 4, up: 13, down: 27 },
                ONode { hdr_info: 6, up: 18, down: 6 },
                ONode { hdr_info: -4, up: 20, down: 25 },
                ONode { hdr_info: 2, up: 16, down: 2 },
                ONode { hdr_info: 7, up: 14, down: 29 },
                ONode { hdr_info: -5, up: 24, down: 29 },
                ONode { hdr_info: 4, up: 21, down: 4 },
                ONode { hdr_info: 5, up: 10, down: 5 },
                ONode { hdr_info: 7, up: 25, down: 7 },
                ONode { hdr_info: -6, up: 27, down: 0 },
            ],
        };
        let items_x = items.clone();
        let opts_x = opts.clone();
        let mut chooser = Mrv {};
        let mut problem = Problem::new(items, opts);
        assert!(problem.next_solution(&mut chooser), "no solution");
        problem.find_options();
        problem.o.sort();
        assert_eq!(problem.o, vec![0, 3, 4], "wrong solution");
        assert!(!problem.next_solution(&mut chooser), "too many solutions");
        assert!(problem.items == items_x, "items not backtracked");
        assert!(problem.opts == opts_x, "options not backtracked");
        assert!(
            problem.l == 0 && problem.restart == false,
            "initial state not restored"
        );
    }

    // TAoCP 4B p. 89
    fn table_2_items() -> Vec<INode> {
        vec![
            INode { left: 3, right: 1 },
            INode { left: 0, right: 2 },
            INode { left: 1, right: 3 },
            INode { left: 2, right: 0 },
            INode { left: 6, right: 5 },
            INode { left: 4, right: 6 },
            INode { left: 5, right: 4 },
        ]
    }

    // TAoCP 4B p. 89
    fn table_2_opts() -> ONodesC {
        ONodesC {
            nodes: vec![
                ONodeC { hdr_info: 0, up: 0, down: 0, color: 0 },
                ONodeC { hdr_info: 3, up: 17, down: 7, color: 0 },
                ONodeC { hdr_info: 2, up: 20, down: 8, color: 0 },
                ONodeC { hdr_info: 2, up: 23, down: 13, color: 0 },
                ONodeC { hdr_info: 4, up: 21, down: 9, color: 0 },
                ONodeC { hdr_info: 3, up: 24, down: 10, color: 0 },
                ONodeC { hdr_info: 0, up: 0, down: 10, color: 0 },
                ONodeC { hdr_info: 1, up: 1, down: 12, color: 0 },
                ONodeC { hdr_info: 2, up: 2, down: 20, color: 0 },
                ONodeC { hdr_info: 4, up: 4, down: 14, color: 0 },
                ONodeC { hdr_info: 5, up: 5, down: 15, color: 1 },
                ONodeC { hdr_info: -1, up: 7, down: 15, color: 0 },
                ONodeC { hdr_info: 1, up: 7, down: 17, color: 0 },
                ONodeC { hdr_info: 3, up: 3, down: 23, color: 0 },
                ONodeC { hdr_info: 4, up: 9, down: 18, color: 1 },
                ONodeC { hdr_info: 5, up: 10, down: 24, color: 0 },
                ONodeC { hdr_info: -2, up: 12, down: 18, color: 0 },
                ONodeC { hdr_info: 1, up: 12, down: 1, color: 0 },
                ONodeC { hdr_info: 4, up: 14, down: 21, color: 2 },
                ONodeC { hdr_info: -3, up: 17, down: 21, color: 0 },
                ONodeC { hdr_info: 2, up: 8, down: 2, color: 0 },
                ONodeC { hdr_info: 4, up: 18, down: 4, color: 1 },
                ONodeC { hdr_info: -4, up: 20, down: 24, color: 0 },
                ONodeC { hdr_info: 3, up: 13, down: 3, color: 0 },
                ONodeC { hdr_info: 5, up: 15, down: 5, color: 2 },
                ONodeC { hdr_info: -5, up: 23, down: 0, color: 0 },
            ],
        }
    }

    // The color field of the header nodes is set but never read
    // except for debugging, so it does not need to be considered
    // when verifying that the nodes are returned to their initial
    // values.
    struct ONodesCEq(usize, ONodesC);
    impl PartialEq for ONodesCEq {
        fn eq(&self, other: &Self) -> bool {
            if !self.0 == other.0 {
                return false;
            }
            let n = self.0 + 2;
            let a_hdrs =
                self.1.nodes[..n].iter().map(|o| (o.hdr_info, o.up, o.down));
            let b_hdrs = other.1.nodes[..n]
                .iter()
                .map(|o| (o.hdr_info, o.up, o.down));
            self.1.nodes[n..] == other.1.nodes[n..] && a_hdrs.eq(b_hdrs)
        }
    }

    #[test]
    // TAoCP 4B p. 89
    fn test_4b_table_2() {
        let items = INodes { nodes: table_2_items(), primary: 3, secondary: 2 };
        let opts = table_2_opts();
        let items_x = items.clone();
        let opts_x = opts.clone();
        let mut chooser = Mrv {};
        let mut problem = Problem::new(items, opts);
        assert!(problem.next_solution(&mut chooser), "no solution");
        problem.find_options();
        problem.o.sort();
        assert_eq!(problem.o, vec![1, 3], "wrong solution");
        assert!(!problem.next_solution(&mut chooser), "too many solutions");
        assert!(problem.items == items_x, "items not backtracked");
        assert!(
            ONodesCEq(5, problem.opts) == ONodesCEq(5, opts_x),
            "options not backtracked"
        );
    }

    #[test]
    // https://cs.stanford.edu/~knuth/papers/Xqueens-and-Xqueenons.pdf
    fn test_xqueens() {
        use core::iter::repeat_n;
        let ms = repeat_n((1, 1), 8)
            .chain(repeat_n((2, 2), 4))
            .chain(repeat_n((0, 2), 12));
        let items = INodeM::make_nodes(24, 0, ms);
        let mut os: Vec<Vec<usize>> = Vec::new();
        for i in 0..2 {
            for j in 0..2 {
                os.push(vec![i, 8 + j, 12 + i + 1 - j, 15 + i + j]);
                os.push(vec![10 + i, 2 + j, 12 + i + 1 - j, 18 + i + j]);
                os.push(vec![4 + i, 8 + j, 21 + i + 1 - j, 18 + i + j]);
                os.push(vec![10 + i, 6 + j, 21 + i + 1 - j, 15 + i + j]);
            }
        }
        let opts = ONode::make_nodes(24, 16, 64, os);
        let mut chooser = Mrv {};
        let mut problem = Problem::new(items, opts);
        let mut solutions: Vec<Vec<isize>> = Vec::new();
        while problem.next_solution(&mut chooser) {
            problem.find_options();
            problem.o.sort();
            solutions.push(problem.o.clone());
        }
        assert_eq!(solutions.len(), 6, "wrong number of solutions");
        assert_eq!(
            solutions,
            vec![
                vec![0, 1, 5, 6, 8, 11, 14, 15],
                vec![0, 2, 5, 7, 9, 11, 12, 14],
                vec![0, 3, 6, 7, 8, 9, 13, 14],
                vec![1, 2, 4, 5, 10, 11, 12, 15],
                vec![1, 3, 4, 6, 8, 10, 13, 15],
                vec![2, 3, 4, 7, 9, 10, 12, 13],
            ]
        );
    }

    #[test]
    fn test_nodes() {
        let items = INode::make_nodes(3, 2);
        assert!(items.nodes == table_2_items(), "incorrect items");
        let opt_spec: Vec<Vec<(usize, isize)>> = vec![
            vec![(0, 0), (1, 0), (3, 0), (4, 1)],
            vec![(0, 0), (2, 0), (3, 1), (4, 0)],
            vec![(0, 0), (3, 2)],
            vec![(1, 0), (3, 1)],
            vec![(2, 0), (4, 2)],
        ];
        let nodes = ONodeC::make_nodes(5, 5, 14, opt_spec);
        assert_eq!(nodes, table_2_opts(), "incorrect options");
    }
}
