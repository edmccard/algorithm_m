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

#[derive(Clone, Debug, Eq, PartialEq)]
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
        let mut nodes = ONodesC {
            n_opts: m,
            nodes: vec![Default::default(); l + m + n + 2],
        };
        nodes.init_links(n, opt_spec);
        nodes
    }
}

#[derive(Clone, Debug)]
pub struct ONodesC {
    nodes: Vec<ONodeC>,
    n_opts: usize,
}

impl ONodesC {
    fn get_node(&mut self, i: usize) -> &mut ONodeC {
        unsafe { self.nodes.get_unchecked_mut(i) }
    }
}

impl PartialEq for ONodesC {
    fn eq(&self, other: &Self) -> bool {
        if self.n_opts != other.n_opts {
            return false;
        }
        let n = self.n_opts + 2;
        let a_hdrs = self.nodes[..n].iter().map(|o| (o.hdr_info, o.up, o.down));
        let b_hdrs =
            other.nodes[..n].iter().map(|o| (o.hdr_info, o.up, o.down));
        self.nodes[n..] == other.nodes[n..] && a_hdrs.eq(b_hdrs)
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

    #[test]
    // TAocp Vol. 4B p. 89
    fn test_nodes() {
        let items = INode::make_nodes(3, 2);
        let inodes = vec![
            INode { left: 3, right: 1 },
            INode { left: 0, right: 2 },
            INode { left: 1, right: 3 },
            INode { left: 2, right: 0 },
            INode { left: 6, right: 5 },
            INode { left: 4, right: 6 },
            INode { left: 5, right: 4 },
        ];
        assert_eq!(items.nodes, inodes, "incorrect items");

        let opt_spec: Vec<Vec<(usize, isize)>> = vec![
            vec![(0, 0), (1, 0), (3, 0), (4, 1)],
            vec![(0, 0), (2, 0), (3, 1), (4, 0)],
            vec![(0, 0), (3, 2)],
            vec![(1, 0), (3, 1)],
            vec![(2, 0), (4, 2)],
        ];
        let opts = ONodeC::make_nodes(5, 5, 14, opt_spec);
        let onodes = vec![
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
        ];
        assert_eq!(opts.nodes, onodes, "incorrect options");
    }
}
