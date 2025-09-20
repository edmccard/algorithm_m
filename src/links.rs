use crate::{IDance, ODance, OptSpec};

pub type Link = usize;
pub type Count = Link;
pub type Data = isize;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INode {
    left: Link,
    right: Link,
}

impl INode {
    pub fn make_nodes(primary: Count, secondary: Count) -> INodes {
        let mut inodes = INodes {
            nodes: vec![Default::default(); (primary + secondary + 2) as usize],
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
    primary: Count,
    secondary: Count,
}

impl INodes {
    fn get_node(&mut self, i: Link) -> &mut INode {
        if cfg!(feature = "unsafe-fast-index") {
            unsafe { self.nodes.get_unchecked_mut(i as usize) }
        } else {
            &mut self.nodes[i as usize]
        }
    }
}

impl IDance for INodes {
    #[inline(always)]
    fn primary(&self) -> Count {
        self.primary
    }

    #[inline(always)]
    fn secondary(&self) -> Count {
        self.secondary
    }

    #[inline(always)]
    fn llink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).left
    }
    #[inline(always)]
    fn rlink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).right
    }

    #[inline(always)]
    fn bound(&mut self, _i: Link) -> Data {
        0
    }

    #[inline(always)]
    fn dec_bound(&mut self, _i: Link) -> Data {
        0
    }

    #[inline(always)]
    fn inc_bound(&mut self, _i: Link) -> Data {
        1
    }

    #[inline(always)]
    fn slack(&mut self, _i: Link) -> Data {
        0
    }

    #[inline(always)]
    fn branch_factor(&mut self, _i: Link) -> Data {
        1
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INodeM {
    left: Link,
    right: Link,
    slack: Data,
    bound: Data,
}

impl INodeM {
    pub fn make_nodes(
        primary: Count,
        secondary: Count,
        ms: impl IntoIterator<Item = (Data, Data)>,
    ) -> INodesM {
        let n = primary + secondary;
        let mut inodes = INodesM {
            nodes: vec![Default::default(); (n + 2) as usize],
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
    primary: Count,
    secondary: Count,
}

impl INodesM {
    fn get_node(&mut self, i: Link) -> &mut INodeM {
        if cfg!(feature = "unsafe-fast-index") {
            unsafe { self.nodes.get_unchecked_mut(i as usize) }
        } else {
            &mut self.nodes[i as usize]
        }
    }
}

impl IDance for INodesM {
    fn primary(&self) -> Count {
        self.primary
    }

    fn secondary(&self) -> Count {
        self.secondary
    }

    fn llink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).left
    }

    fn rlink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).right
    }

    fn bound(&mut self, i: Link) -> Data {
        self.get_node(i).bound
    }

    fn dec_bound(&mut self, i: Link) -> Data {
        let node = self.get_node(i);
        node.bound -= 1;
        node.bound
    }

    fn inc_bound(&mut self, i: Link) -> Data {
        let node = self.get_node(i);
        node.bound += 1;
        node.bound
    }

    fn slack(&mut self, i: Link) -> Data {
        self.get_node(i).slack
    }

    fn branch_factor(&mut self, i: Link) -> Data {
        let node = self.get_node(i);
        node.bound.saturating_sub(node.slack)
    }
}

impl OptSpec for Count {
    fn get_item(&self) -> Count {
        *self
    }
    fn get_color(&self) -> Data {
        0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ONode {
    hdr_info: Data,
    up: Link,
    down: Link,
}

impl ONode {
    pub fn make_nodes(
        n: Count,
        m: Count,
        l: Count,
        opt_spec: impl IntoIterator<Item = impl IntoIterator<Item = Count>>,
    ) -> ONodes {
        let mut nodes = ONodes {
            nodes: vec![Default::default(); (l + m + n + 2) as usize],
        };
        nodes.init_links(n, opt_spec);
        nodes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ONodes {
    nodes: Vec<ONode>,
}

impl ONodes {
    fn get_node(&mut self, i: Link) -> &mut ONode {
        unsafe { self.nodes.get_unchecked_mut(i as usize) }
    }
}

impl ODance for ONodes {
    type Spec = Count;

    #[inline(always)]
    fn olen(&mut self, i: Link) -> &mut Data {
        &mut self.get_node(i).hdr_info
    }

    #[inline(always)]
    fn top(&mut self, i: Link) -> &mut Data {
        &mut self.get_node(i).hdr_info
    }

    #[inline(always)]
    fn ulink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).up
    }

    #[inline(always)]
    fn dlink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).down
    }

    #[inline(always)]
    fn get_color(&mut self, _i: Link) -> Data {
        0
    }

    #[inline(always)]
    fn set_color(&mut self, _i: Link, _c: Data) {}
}

impl OptSpec for (Count, Data) {
    fn get_item(&self) -> Count {
        self.0
    }
    fn get_color(&self) -> Data {
        self.1
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ONodeC {
    hdr_info: Data,
    up: Link,
    down: Link,
    color: Data,
}

impl ONodeC {
    pub fn make_nodes(
        n: Count,
        m: Count,
        l: Count,
        opt_spec: impl IntoIterator<Item = impl IntoIterator<Item = (Count, Data)>>,
    ) -> ONodesC {
        let mut nodes = ONodesC {
            n_opts: m,
            nodes: vec![Default::default(); (l + m + n + 2) as usize],
        };
        nodes.init_links(n, opt_spec);
        nodes
    }
}

#[derive(Clone, Debug)]
pub struct ONodesC {
    nodes: Vec<ONodeC>,
    n_opts: Count,
}

impl ONodesC {
    fn get_node(&mut self, i: Link) -> &mut ONodeC {
        unsafe { self.nodes.get_unchecked_mut(i as usize) }
    }
}

impl PartialEq for ONodesC {
    fn eq(&self, other: &Self) -> bool {
        if self.n_opts != other.n_opts {
            return false;
        }
        let n = self.n_opts + 2;
        let a_hdrs = self.nodes[..n as usize]
            .iter()
            .map(|o| (o.hdr_info, o.up, o.down));
        let b_hdrs = other.nodes[..n as usize]
            .iter()
            .map(|o| (o.hdr_info, o.up, o.down));
        self.nodes[n as usize..] == other.nodes[n as usize..]
            && a_hdrs.eq(b_hdrs)
    }
}

impl ODance for ONodesC {
    type Spec = (Count, Data);

    fn olen(&mut self, i: Link) -> &mut Data {
        &mut self.get_node(i).hdr_info
    }
    fn top(&mut self, i: Link) -> &mut Data {
        &mut self.get_node(i).hdr_info
    }
    fn ulink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).up
    }
    fn dlink(&mut self, i: Link) -> &mut Link {
        &mut self.get_node(i).down
    }

    fn get_color(&mut self, i: Link) -> Data {
        self.get_node(i).color
    }
    fn set_color(&mut self, i: Link, c: Data) {
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

        let opt_spec: Vec<Vec<(Count, Data)>> = vec![
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
