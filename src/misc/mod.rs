use rand::seq::SliceRandom;
use std::io::{self, BufRead};

struct NonCopy(i32);

fn mapping() {
    let v = vec![NonCopy(1), NonCopy(2), NonCopy(100)];
    let mapped = v.iter().map(|i| match i {
        NonCopy(1) => "One",
        NonCopy(2) => "Two",
        NonCopy(_) => "Many",
    });
}

fn ref_test() {
    let mut val = 1;
    let two = vec![1u8, 2u8];
    let three: u32 = 3;
    let mut r = &mut val;
    r = &mut 4;
}

/// For automatic Option<T> ->
trait OptionToResult<T, E> {}

pub(crate) fn process_lines(reader: impl BufRead + Sized) {}

struct Latin {}
struct Greek {}
struct Arabic {}
struct Chinese {}
trait Alphabet {}
impl Alphabet for Latin {}
impl Alphabet for Greek {}
impl Alphabet for Arabic {}
impl Alphabet for Chinese {}

pub fn collection_type_hint_applies_to_trait_items() {
    let latin = Latin {};
    let greek = Greek {};
    let alphabets: Vec<&dyn Alphabet> = vec![&latin, &greek];

    alphabets.choose(&mut rand::thread_rng()).unwrap();
}

// TODO a trait & implement it 2x for the same
trait DummyIterator<T> {}
// struct

// fn -> Option<Self::Item>
pub fn split_slice_with_questionmark_operator() -> Option<i32> {
    let array = [1, 2, 3];
    let (head, _) = array.split_first()?;
    Some(*head)
}

// Generic associated types
trait GatTrait {
    type Item<T, U>;
}

#[test]
fn vector_access_with_questionmark_operator() {
    let mut v = vec![1, 2];
    v.push(3);
    v.pop();
    //v.shrink_to(min_capacity);

    // Vec.into_boxed_slice() uses ManuallyDrop. See also mem::forget.
    let boxed = v.into_boxed_slice();
    boxed.len();
}
