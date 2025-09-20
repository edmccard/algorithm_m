use crate::{Count, Data, Link};

pub trait Items {
    fn primary(&self) -> Count;
    fn secondary(&self) -> Count;

    fn llink(&mut self, i: Link) -> &mut Link;
    fn rlink(&mut self, i: Link) -> &mut Link;

    fn bound(&mut self, i: Link) -> Data;
    fn dec_bound(&mut self, i: Link) -> Data;
    fn inc_bound(&mut self, i: Link) -> Data;
    fn slack(&mut self, i: Link) -> Data;
    fn branch_factor(&mut self, i: Link) -> Data;

    fn init_links(&mut self) {
        let n1 = self.primary();
        let n = self.primary() + self.secondary();
        for i in (1 as Link)..=n {
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
