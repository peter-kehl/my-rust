#![allow(unstable_features)]
#![feature(never_type)]
#![feature(try_blocks)]

#[test]
fn cant_reborrow_mutable_and_then_both_as_shared() {
    let mut val = 1;
    let mut_ref1 = &mut val;
    let mut_ref2 = &mut *mut_ref1;
    //print!( "{}", mut_ref1 == mut_ref2);
}

#[test]
fn try_block() {
    use std::num::ParseIntError;

    let result: Result<i32, ParseIntError> =
        try { "1".parse::<i32>()? + "2".parse::<i32>()? + "3".parse::<i32>()? };
    assert_eq!(result, Ok(6));
}

#[test]
fn collecting() {
    let mapped = vec![1, 2, 3].into_iter().map(|n| <Result<u32, !>>::Ok(n));
    let mapped2 = mapped.clone();

    let collected: Vec<Result<u32, !>> = mapped.collect();

    let res: Result<Vec<u32>, !> = mapped2.collect(); // <- from_iter
    assert!(res.is_ok());
    let res: Result<Vec<u32>, !> = vec![1, 2, 3].into_iter().map(|n| Ok(n)).collect();
    assert!(res.is_ok());

    let mapped_err = vec![1, 2, 3]
        .into_iter()
        .map(|n| <Result<u32, ()>>::Err(()));
    let mapped_err2 = mapped_err.clone();

    let collected_err: Vec<Result<u32, ()>> = mapped_err.collect();
    let every_item_is_err: bool = collected_err
        .into_iter()
        .fold(true, |acc, item| acc && item.is_err());
    assert!(every_item_is_err);

    let res_err: Result<Vec<u32>, ()> = mapped_err2.collect();
    assert!(res_err.is_err());

    Some(1).into_iter().for_each(|n| println!("{}", n));
}
