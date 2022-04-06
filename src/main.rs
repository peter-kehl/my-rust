#![allow(unstable_features)]
#![feature(generic_associated_types)]
//#![feature(array_map)]

// ------
mod copy_clone;
mod misc;

use copy_clone::WithPrivateField;

use std::ffi::CStr;
use std::os::raw::c_char;
use Mood::Happy;

fn lifetimes() {
    let mut x = 1;
    let mut r = &x;

    let mut y = 2;
    r = &y;

    let copied = *r;
}

fn cant_use_collections_after_map() {
    let range = (-1i16..1i16);
    let result: Vec<_> = range.map(|x| ()).collect();
    //let another = range; // CAN't - consumed!
}

const ARRAY: [i64; 2] = [1, 2];

fn array_into_iterator() {
    // There may be a workaround in Rust 2021?
    std::array::IntoIter::new(ARRAY);
}

fn multi_mut() {
    let mut val = 1;
    let rf = &mut &mut val;
}

trait CheckedAdd<T: Copy> {
    fn add(left: T, right: T) -> Option<T> {
        todo!();
    }
}

struct WithCheckedAdd<T> {
    field: T,
}

impl<T: Copy> CheckedAdd<T> for WithCheckedAdd<T> {
    fn add(left: T, right: T) -> Option<T> {
        todo!();
    }
}

impl WithCheckedAdd<u64> {
    fn add(self, left: u64, right: u64) -> Option<u64> {
        todo!();
    }
}

impl WithCheckedAdd<u128> {
    fn add(self, left: u128, right: u128) -> Option<u128> {
        todo!();
    }
}

impl<T: Copy> WithCheckedAdd<T> {
    fn use_add(left: T, right: T) -> Option<T> {
        Self::add(left, right)
    }
}
// -------

static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn raw_string() {
    let c_ptr = &C as *const u8 as *const c_char;
    let c = unsafe { CStr::from_ptr(c_ptr).to_string_lossy() };
}

//struct S

enum Mood {
    Happy,
}
fn f() {
    let mood = Happy;
}

fn main_old() {
    // fails: WithPrivateField {};
    let mut with_private_field = WithPrivateField::new();

    let somevalue = 123;
    println!("Hello, world..! {}", somevalue);
}

// For Aussie
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes: Vec<u8> = vec![
        87, 104, 101, 110, 32, 100, 111, 32, 119, 101, 32, 109, 101, 101, 116, 63,
    ];
    println!("{}", String::from_utf8(bytes)?);
    Ok(())
}

fn loop_invariant() {
    let mut nums = vec![1, 2, 3];
    for i in 0..6 {
        let _ = nums[i];
    }
    for (i, _value) in nums.iter().enumerate() {
        nums.remove(i); // CAN modify ONLY because we return right after.
        return;
    }
}

#[derive(Debug)]
struct NotCopy {}

fn underscore() {
    let x = NotCopy {};
    //let i = x;
    //drop(i);
    let _ = x;
    println!("{:?}", x);
    let _ = x;
}

#[derive(Debug)]
enum Count {
    Zero,
    One,
    Many(usize),
}

fn example(c: Count) {
    use Count::*;
    match c {
        x @ Zero | x @ One => println!("{:?}", x), // what is the type of `x` here?
        x => println!("{:?}", x),
    }
}

// ----- aaron_abramov
use std::collections::HashMap;
use std::thread;
use std::time::Instant;
const NUM_ELEMENTS: usize = 1000000;
type HeavyThings = HashMap<usize, Vec<usize>>;
fn main_aaron_abramov() {
    let heavy_things_1 = make_heavy_things();
    let heavy_things_2 = make_heavy_things();
    let len = log_time("drop in another thread", || {
        fn_that_drops_heavy_things_in_another_thread(heavy_things_2)
    });
    assert_eq!(len, NUM_ELEMENTS);
    let len = log_time("drop in this thread", || {
        fn_that_drops_heavy_things(heavy_things_1)
    });
    assert_eq!(len, NUM_ELEMENTS);
}
fn make_heavy_things() -> HeavyThings {
    (1..=NUM_ELEMENTS).map(|v| (v, vec![v])).collect()
}
fn fn_that_drops_heavy_things(things: HeavyThings) -> usize {
    things.len()
}
fn fn_that_drops_heavy_things_in_another_thread(things: HeavyThings) -> usize {
    let len = things.len();
    thread::spawn(move || drop(things));
    len
}
fn log_time<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
    let time = Instant::now();
    let result = f();
    println!("{} {:?}", name, time.elapsed());
    result
}

//----
// from fosskers
pub(crate) trait ResultVoid<E, R> {
    fn void(self) -> Result<(), R>
    where
        R: From<E>;
}

impl<T, E, R> ResultVoid<E, R> for Result<T, E> {
    fn void(self) -> Result<(), R>
    where
        R: From<E>,
    {
        match self {
            Ok(_) => Ok(()),
            Err(e) => Err(From::from(e)),
        }
    }
}
