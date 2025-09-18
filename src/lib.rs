pub mod links;

pub struct Problem<I, O>
where
    I: IDance,
    O: ODance,
{
    items: I,
    opts: O,
    x: Vec<usize>,
    ft: Vec<usize>,
    o: Vec<isize>,
    l: usize,
    i: usize,
    restart: bool,
}

impl<I, O> Problem<I, O>
where
    I: IDance,
    O: ODance,
{
    pub fn new(items: I, opts: O) -> Problem<I, O> {
        Problem {
            items,
            opts,
            x: Vec::new(),
            ft: Vec::new(),
            o: Vec::new(),
            l: 0,
            i: 0,
            restart: false,
        }
    }

    pub fn next_solution<C: Choose<I, O>>(&mut self, chooser: &mut C) -> bool {
        let mut l = self.l;
        let mut i = self.i;

        let n = self.items.primary() + self.items.secondary();
        let n1 = self.items.primary();

        loop {
            if self.restart {
                self.restart = false;
            // goto M9
            } else {
                // M2
                if *self.items.rlink(0) == 0 {
                    self.l = l;
                    self.i = i;
                    self.restart = true;
                    return true;
                } else {
                    if self.x.len() == l {
                        self.x.push(0);
                        self.ft.push(0);
                    }
                    // M3
                    i = chooser.choose(&mut self.items, &mut self.opts);
                    if (1 + *self.opts.olen(i)) > self.items.branch_factor(i) {
                        // M4
                        self.x[l] = *self.opts.dlink(i);
                        if self.items.dec_bound(i) == 0 {
                            self.cover(i);
                            if self.items.slack(i) != 0 {
                                self.ft[l] = self.x[l];
                            }
                        } else {
                            self.ft[l] = self.x[l];
                        }

                        // M5,M6
                        if self.try_item(i, self.x[l], n1) {
                            l += 1;
                            continue;
                            // go to M2
                        } else {
                            // M8
                            self.restore_item(i, self.ft[l], n);
                        }
                    }
                    // go to M9
                }
            }
            loop {
                // M9
                if l == 0 {
                    return false;
                }
                l -= 1;
                if self.x[l] > n {
                    i = *self.opts.top(self.x[l]) as usize;
                    // M7
                    let mut p = self.x[l] - 1;
                    while p != self.x[l] {
                        let j = *self.opts.top(p);
                        if j <= 0 {
                            p = *self.opts.dlink(p);
                        } else if j as usize <= n1 {
                            p -= 1;
                            if self.items.inc_bound(j as usize) == 1 {
                                self.uncover(j as usize);
                            }
                        } else {
                            self.uncommit(p, j as usize);
                            p -= 1;
                        }
                    }
                    self.x[l] = *self.opts.dlink(self.x[l]);
                    // M5,M6
                    if self.try_item(i, self.x[l], n1) {
                        l += 1;
                        break;
                        // next: M2
                    }
                    // next: M8
                } else {
                    i = self.x[l];
                    let p = *self.items.llink(i);
                    let q = *self.items.rlink(i);
                    *self.items.rlink(p) = i;
                    *self.items.llink(q) = i;
                    // next: M8
                }
                // M8
                self.restore_item(i, self.ft[l], n);
            }
        }
    }

    pub fn find_options(&mut self) {
        let n = self.items.primary() + self.items.secondary();
        self.o.clear();
        for xj in &self.x[..self.l] {
            let mut r = *xj;
            if r <= n {
                // TODO: somehow report this
                continue;
            }
            while *self.opts.top(r) >= 0 {
                r += 1;
            }
            // Internal option indexes are 1-based
            self.o.push(-*self.opts.top(r) - 1);
        }
    }

    fn try_item(&mut self, i: usize, xl: usize, n1: usize) -> bool {
        // M5
        if self.items.slack(i) == 0 && self.items.bound(i) == 0 {
            if xl == i {
                return false;
                // go to M8
            }
        // go to M6
        } else if *self.opts.olen(i)
            <= (self.items.bound(i) - self.items.slack(i))
        {
            return false;
            // go to M8
        } else if xl != i {
            self.tweak(xl, i);
        } else if self.items.bound(i) != 0 {
            let p = *self.items.llink(i);
            let q = *self.items.rlink(i);
            *self.items.rlink(p) = q;
            *self.items.llink(q) = p;
        }
        // M6
        if xl != i {
            let mut p = xl + 1;
            while p != xl {
                let j = *self.opts.top(p);
                if j <= 0 {
                    p = *self.opts.ulink(p);
                } else if j as usize <= n1 {
                    p += 1;
                    if self.items.dec_bound(j as usize) == 0 {
                        self.cover(j as usize);
                    }
                } else {
                    self.commit(p, j as usize);
                    p += 1;
                }
            }
        }
        true
    }

    fn restore_item(&mut self, i: usize, ftl: usize, n: usize) {
        if self.items.bound(i) == 0 && self.items.slack(i) == 0 {
            self.uncover(i);
        } else if self.items.bound(i) == 0 {
            self.untweak_b(ftl, n);
        } else {
            self.untweak(ftl, n);
        }
        self.items.inc_bound(i);
    }

    fn commit(&mut self, p: usize, j: usize) {
        if self.opts.get_color(p) == 0 {
            self.cover(j);
        }
        if self.opts.get_color(p) > 0 {
            self.purify(p);
        }
    }

    fn uncommit(&mut self, p: usize, j: usize) {
        if self.opts.get_color(p) == 0 {
            self.uncover(j)
        }
        if self.opts.get_color(p) > 0 {
            self.unpurify(p);
        }
    }

    fn cover(&mut self, i: usize) {
        // TODO: increment updates
        let mut p = *self.opts.dlink(i);
        while p != i {
            self.hide(p);
            p = *self.opts.dlink(p);
        }
        let l = *self.items.llink(i);
        let r = *self.items.rlink(i);
        *self.items.rlink(l) = r;
        *self.items.llink(r) = l;
    }

    fn uncover(&mut self, i: usize) {
        let l = *self.items.llink(i);
        let r = *self.items.rlink(i);
        *self.items.rlink(l) = i;
        *self.items.llink(r) = i;
        let mut p = *self.opts.ulink(i);
        while p != i {
            self.unhide(p);
            p = *self.opts.ulink(p);
        }
    }

    fn hide(&mut self, p: usize) {
        let mut q = p + 1;
        while q != p {
            let x = *self.opts.top(q);
            let u = *self.opts.ulink(q);
            let d = *self.opts.dlink(q);
            if x <= 0 {
                q = u;
            } else {
                if self.opts.get_color(q) >= 0 {
                    *self.opts.dlink(u) = d;
                    *self.opts.ulink(d) = u;
                    // TODO: increment updates
                    *self.opts.olen(x as usize) -= 1;
                }
                q += 1;
            }
        }
    }

    fn unhide(&mut self, p: usize) {
        let mut q = p - 1;
        while q != p {
            let x = *self.opts.top(q);
            let u = *self.opts.ulink(q);
            let d = *self.opts.dlink(q);
            if x <= 0 {
                q = d;
            } else {
                if self.opts.get_color(q) >= 0 {
                    *self.opts.dlink(u) = q;
                    *self.opts.ulink(d) = q;
                    *self.opts.olen(x as usize) += 1;
                }
                q -= 1;
            }
        }
    }

    fn purify(&mut self, p: usize) {
        let c = self.opts.get_color(p);
        let i = *self.opts.top(p) as usize;
        self.opts.set_color(i, c); // HMM not needed?
        let mut q = *self.opts.dlink(i);
        while q != i {
            if self.opts.get_color(q) == c {
                self.opts.set_color(q, -1);
            } else {
                self.hide(q)
            }
            q = *self.opts.dlink(q);
        }
    }

    fn unpurify(&mut self, p: usize) {
        let c = self.opts.get_color(p);
        let i = *self.opts.top(p) as usize;
        let mut q = *self.opts.ulink(i);
        while q != i {
            if self.opts.get_color(q) < 0 {
                self.opts.set_color(q, c);
            } else {
                self.unhide(q);
            }
            q = *self.opts.ulink(q);
        }
    }

    fn tweak(&mut self, x: usize, p: usize) {
        // "We will tweak(x, p) only when x = DLINK(p) and p = ULINK(x)."
        if self.items.bound(p) != 0 {
            self.hide(x);
        }
        let d = *self.opts.dlink(x);
        *self.opts.dlink(p) = d;
        *self.opts.ulink(d) = p;
        *self.opts.olen(p) -= 1;
    }

    fn untweak(&mut self, ftl: usize, n: usize) {
        let p = if ftl <= n {
            ftl
        } else {
            *self.opts.top(ftl) as usize
        };
        let mut x = ftl;
        let mut y = p;
        let z = *self.opts.dlink(p);
        *self.opts.dlink(p) = x;
        let mut k = 0;
        while x != z {
            *self.opts.ulink(x) = y;
            k += 1;
            self.unhide(x);
            y = x;
            x = *self.opts.dlink(x);
        }
        *self.opts.ulink(z) = y;
        *self.opts.olen(p) += k;
    }

    fn untweak_b(&mut self, ftl: usize, n: usize) {
        let p = if ftl <= n {
            ftl
        } else {
            *self.opts.top(ftl) as usize
        };
        let mut x = ftl;
        let mut y = p;
        let z = *self.opts.dlink(p);
        *self.opts.dlink(p) = x;
        let mut k = 0;
        while x != z {
            *self.opts.ulink(x) = y;
            k += 1;
            y = x;
            x = *self.opts.dlink(x);
        }
        *self.opts.ulink(z) = y;
        *self.opts.olen(p) += k;
        self.uncover(p);
    }
}

pub trait IDance {
    fn primary(&self) -> usize;
    fn secondary(&self) -> usize;

    fn llink(&mut self, i: usize) -> &mut usize;
    fn rlink(&mut self, i: usize) -> &mut usize;

    fn bound(&self, i: usize) -> isize;
    fn dec_bound(&mut self, i: usize) -> isize;
    fn inc_bound(&mut self, i: usize) -> isize;
    fn slack(&self, i: usize) -> isize;
    fn branch_factor(&self, i: usize) -> isize;

    fn init_links(&mut self) {
        let n1 = self.primary();
        let n = self.primary() + self.secondary();
        for i in 1..=n {
            *self.llink(i) = i - 1;
            *self.rlink(i - 1) = i;
        }
        *self.llink(n + 1) = n;
        *self.rlink(n) = n + 1;
        *self.llink(n1 + 1) = n + 1;
        *self.rlink(n + 1) = n1 + 1;
        *self.llink(0) = n1;
        *self.rlink(n1) = 0;
    }
}

pub trait OptSpec {
    fn get_item(&self) -> usize;
    fn get_color(&self) -> isize;
}

pub trait ODance {
    type Spec: OptSpec;

    fn olen(&mut self, i: usize) -> &mut isize;
    fn top(&mut self, i: usize) -> &mut isize;
    fn ulink(&mut self, i: usize) -> &mut usize;
    fn dlink(&mut self, i: usize) -> &mut usize;

    fn get_color(&self, i: usize) -> isize;
    fn set_color(&mut self, i: usize, c: isize);

    // TODO: allow for randomization
    fn init_links(
        &mut self,
        n: usize,
        opt_spec: impl IntoIterator<Item = impl IntoIterator<Item = Self::Spec>>,
    ) {
        for i in 1..=n {
            *self.ulink(i) = i;
            *self.dlink(i) = i;
        }
        let mut m: isize = 0;
        let mut p: usize = n + 1;
        for opts in opt_spec.into_iter() {
            let mut k = 0;
            for opt in opts.into_iter() {
                // Internal item numbers are 1-based.
                let ij = opt.get_item() + 1;
                k += 1;
                *self.olen(ij) += 1;
                let q = *self.ulink(ij);
                *self.ulink(p + k) = q;
                *self.dlink(q) = p + k;
                *self.dlink(p + k) = ij;
                *self.ulink(ij) = p + k;
                *self.top(p + k) = ij as isize;
                let c = opt.get_color();
                self.set_color(p + k, c);
            }
            m += 1;
            *self.dlink(p) = p + k;
            p = p + k + 1;
            *self.top(p) = -m;
            *self.ulink(p) = p - k;
        }
    }
}

// TODO: preferences and randomization
pub trait Choose<I: IDance, O: ODance> {
    // TODO: return the score for the item
    fn choose(&mut self, items: &mut I, opts: &mut O) -> usize;
}

pub struct Mrv {}

impl<I: IDance, O: ODance> Choose<I, O> for Mrv {
    fn choose(&mut self, items: &mut I, opts: &mut O) -> usize {
        let mut min = isize::MAX;
        let mut p = *items.rlink(0);
        let mut i = p;
        while p != 0 {
            if *opts.olen(p) == 0 {
                break;
            } else if *opts.olen(p) < min {
                min = *opts.olen(p);
                i = p;
            }
            p = *items.rlink(p);
        }
        i
    }
}
