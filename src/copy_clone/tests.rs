#![cfg(test)]

use super::{ContainsString, DeriveCopy};

#[test]
fn slice_into_iter() {
    let indices = [0usize, 0, 1, 0, 1, 2, 1];
    //let iter: IntoIterator<Item = usize> = indices.into_iter();
    let into_iter = indices.into_iter();
    println!("slice.into_iter(): {:#?}", into_iter);

    let iter = indices.iter();
    println!("slice.iter(): {:#?}", iter);
    //let a: Option<std::slice::Iter<'_, usize>> = None;
}

#[test]
fn for_loop_over_slice() {
    let items = ['a', 'b', 'c'];
    for c in &items[..] {
        println!("for_loop_over_slice: {}", c);
    }
}

#[test]
fn derive_copy() {
    let mut one = DeriveCopy { i: -1, s: "hi" };
    let mut two = one;

    println!("one: {:?}", one);
    println!("two: {:#?}", two); // pretty print
}

#[test]
fn contains_string() {
    let original = ContainsString { s: "hi".into() };
    let clone = original.clone();
    println!("original.s (String): {}", &original.s as *const _ as usize);
    println!("clone.s (String): {}", &clone.s as *const _ as usize);
}
