use crate::links::INodesM;
use crate::{Count, Data, Items, Link, ODance};

pub trait Choose<I: Items> {
    fn choose<O>(&mut self, items: &mut I, opts: &mut O) -> Link
    where
        O: ODance;
}

pub trait Preference {
    fn is_preferred(&self, p: Link) -> bool;
}

pub trait Tiebreak {
    type I: Items;
    fn replace<O>(
        &self,
        r: usize,
        i: Link,
        p: Link,
        items: &mut Self::I,
        opts: &mut O,
    ) -> bool
    where
        O: ODance;
}

pub struct MRVChooser<P, T>
where
    P: Preference,
    T: Tiebreak,
{
    prefer: P,
    tiebreak: T,
}

impl<P, T> MRVChooser<P, T>
where
    P: Preference,
    T: Tiebreak,
{
    pub fn new(prefer: P, tiebreak: T) -> MRVChooser<P, T> {
        MRVChooser { prefer, tiebreak }
    }

    fn choose_mrv<I, O>(&mut self, items: &mut T::I, opts: &mut O) -> Link
    where
        O: ODance,
    {
        let mut min = Data::MAX;
        let mut p = *items.rlink(0);
        let mut i = p;
        let mut r: usize = 0;
        while p != 0 {
            let olen = if self.prefer.is_preferred(p) {
                *opts.olen(p)
            } else {
                *opts.olen(p) + (opts.size() as Data)
            };
            let curr = (olen + 1).saturating_sub(items.branch_factor(p));
            if curr < min {
                r = 1;
                min = curr;
                i = p;
            } else if curr == min {
                r += 1;
                if self.tiebreak.replace(r, i, p, items, opts) {
                    min = curr;
                    i = p;
                }
            }
            p = *items.rlink(p);
        }
        i
    }
}

impl<P: Preference, T: Tiebreak> Choose<T::I> for MRVChooser<P, T> {
    fn choose<O>(&mut self, items: &mut T::I, opts: &mut O) -> Link
    where
        O: ODance,
    {
        self.choose_mrv::<T::I, O>(items, opts)
    }
}

pub struct NoPreference();
impl Preference for NoPreference {
    fn is_preferred(&self, _p: Count) -> bool {
        true
    }
}

pub fn no_preference() -> NoPreference {
    NoPreference()
}

use std::marker::PhantomData;
pub struct FirstWins<T>(PhantomData<T>)
where
    T: Items;
impl<T: Items> Tiebreak for FirstWins<T> {
    type I = T;
    fn replace<O>(
        &self,
        _r: usize,
        _i: Link,
        _p: Link,
        _items: &mut Self::I,
        _opts: &mut O,
    ) -> bool
    where
        O: ODance,
    {
        false
    }
}

pub fn first_wins<T: Items>() -> FirstWins<T> {
    FirstWins(PhantomData)
}

pub struct KnuthTiebreak();
impl Tiebreak for KnuthTiebreak {
    type I = INodesM;
    fn replace<O>(
        &self,
        _r: usize,
        i: Link,
        p: Link,
        items: &mut Self::I,
        opts: &mut O,
    ) -> bool
    where
        O: ODance,
    {
        items.slack(p) < items.slack(i)
            || (items.slack(p) == items.slack(i)
                && *opts.olen(p) > *opts.olen(i))
    }
}
