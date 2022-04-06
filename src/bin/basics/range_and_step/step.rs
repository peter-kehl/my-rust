// rustup toolchain install nightly
// rustup run nightly rustc step.rs
#![feature(step_trait)] //For unstable feature
extern crate core;
use core::ops::RangeBounds;
use core::ops::Bound;
use core::iter::Step;
use std::cmp::Ordering;

#[derive(Debug,Clone,PartialEq)]
struct S(usize);

impl Step for S {
    fn steps_between(start: &S, end: &S) -> Option<usize> {
        if end.0>start.0 {
            Some( end.0-start.0 )
        }
        else {
            None
        }
    }
    fn replace_one(&mut self) -> Self {
        //won't compile
        //self.0= 1;
        // *self
        panic!("TODO");
    }
    fn replace_zero(&mut self) -> Self {
        //won't compile
        //self.0= 0;
        // *self
        panic!("TODO");
    }
    fn add_one(&self) -> Self { S(self.0+1) }
    fn sub_one(&self) -> Self { S(self.0-1) }
    fn add_usize(&self, n: usize) -> Option<Self> {
        if self.0+n<std::usize::MAX {
            Some( S(self.0+n) )
        }
        else {
            None
        }
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some( self.0.cmp(&other.0) )
    }
}

// This doesn't apply to: for i in a..b
impl RangeBounds<usize> for S {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included( &self.0 )
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded( &self.0 )
    }
}

fn main() {
    let a=S(1); let b=S(4);
    //println!( "{:?}", a..b);
    for i in a..b {
        println!( "{:?}", i );
    }
    // TODO how to access a.0 in another way?
}
