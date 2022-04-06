#[macro_use]
extern crate lazy_static;
use std::vec::Vec;

mod algorithms;
mod serde_example;

trait SliceIterable<T: Default> {
    fn iter<'a>(&'a self) -> core::slice::Iter<'a, T>;
}

struct SliceIterableImpl<T: Default> {
    arr: [T; 5],
}
impl<T: Default> SliceIterable<T> for SliceIterableImpl<T> {
    fn iter<'a>(&'a self) -> core::slice::Iter<'a, T> {
        self.arr.iter()
    }
}

trait Pet {}
fn intrdouce_yourself(p: &dyn Pet) {}

/// Use [`i32`] and [`Vec`] e.g. `Vec::<String>`
/// ```
/// if true {
///    "Hello"
/// } else {
///    "Bye"
/// };
///
/// ```
pub fn fxx() {}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    trait Slice<T: Clone> {
        fn from(value: T) -> Self;
    }

    
    /// Why don't we name this `SliceImpl` and why don't we rename `SliceImpl` 
    /// to `SliceImplCopy`? Yes, it could be less typing, because usertypes
    /// that are not `Copy` are more common. However, it could lead to laziness/
    /// not noticing/forgetting to use `SliceImplCopy` whenever possible.
    /// Especially so because then any `Copy` type could be stored in either
    /// `SliceImpl` or `SliceImplCopy`. Storing `Copy` in Clone-friendly
    /// storage would work, but it would be less efficient than storing it in a
    /// specialized `Copy` storage. Even more so with default values (which, if
    /// primitive and if stored in specialized `Copy` storage, could be
    /// optimized away - so the
    /// memory wouldn't be used until written to it later).
    /// However, if we have `SliceImpl` work for `Copy` only, then it's unlikely
    /// anyone would use it for `Clone` types (even though they could). And we
    /// have `SliceImplClone` for `Clone` types.
    struct SliceImpl<T: Copy, const N: usize> {
        _arr: [T; N],
    }
    struct SliceImplClone<T: Clone, const N: usize> {
        _arr: [T; N],
    }
    /// x
    /// xDefault or xDef
    /// xClone
    /// xCloneDefault or xClondeDef
    /// - self.arr = Default::default()
    impl <T: Copy, const N: usize> Slice<T> for SliceImpl<T, N> {
        fn from(_value: T) -> Self {
            todo!() // MaybeUninit
        }
    }
    impl <T: Clone, const N: usize> Slice<T> for SliceImplClone<T, N> {
        fn from(_value: T) -> Self {
            todo!() // MaybeUninit
        }
    }

    /// "newtype" wrapper, so that we can implement `FromIterator` for arrays
    ///  of any type (that is `Clone`).
    struct CloneableArray<T, const N: usize>([T; N]);

    impl<T: Clone, const N: usize> FromIterator<T> for CloneableArray<T, N> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let t: T;
            panic!()
        }
    }

    // The following fails - an implementation conflict!!
    /*impl<T: Copy, const N: usize> FromIterator<T> for CloneableArray<T, N> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let t: T;
            panic!()
        }
    }*/

    trait Parent {
        fn f(&self);
    }
    // Child has a separate method with the same name, independent of Parent!
    trait Child: Parent {
        fn f(&self) {}
    }
    struct S {}
    /*impl Child for S {
    }

    fn test_f() {
        let s = S {};
        let child: &dyn Child = &S {};
        let parent: &dyn Parent = child as &Parent;
        parent.f();
    }*/

    // Based on the below, implement FromIterator for CloneableForArray.

    // @TODO Do NOT support Pin
    trait CloneableForArray: Clone {
        fn array_from_value<const N: usize>(value: &Self) -> [Self; N] {
            todo!()
        } //TODO also a similar method but with passing the ownership? Does it matter?

        fn array_from_call<const N: usize>(f: impl Fn() -> Self) -> [Self; N] {
            todo!()
        }
        /// For creating an array from (items from) the given iterator.
        /// However, if you know your data source to be an array, call
        /// `array_from_array` instead.
        /// Param `default` is used only if `Self` is not `Copy`.
        fn array_from_iter<const N: usize>(
            default: Option<Self>,
            iter: impl Iterator<Item = Self>,
        ) -> [Self; N] {
            todo!()
        }
        /// A shortcut, so that if copying an array to an array, we don't need
        /// any iterator.
        fn array_from_array<const N: usize>() -> [Self; N] {
            todo!()
        }
        fn write(&self, maybe_uninit: &mut MaybeUninit<Self>) {
            //maybe_uninit.write(self.clone());
        }
    }
    impl<T> CloneableForArray for T
    where
        T: Clone,
    {
        fn array_from_value<const N: usize>(value: &Self) -> [Self; N] {
            todo!()
        }
    }

    /*impl<T> CloneableForArray for T
    where
        T: Copy,
    {
        fn array_from_value<const N: usize>(value: &Self) -> [Self; N] {
            todo!()
        }
    }*/

    #[derive(Clone)]
    struct CloneButNoCopy {}

    fn use_cloneable_for_array() {
        let copy = 'a';
    }

    trait T {}
    //struct SS {}
    impl<TP> T for TP where TP: Clone + Copy {}

    /// TO TRY:
    /// SliceStorage { ArrayCopy(...), ArrayClone(...)}



    fn test_cloneable_for_array() {
        let copy = <char as CloneableForArray>::array_from_value::<4>(&'a');
        let clone = <Vec<char> as CloneableForArray>::array_from_value::<4>(&vec!['a']);
    }

    use std::{collections::HashMap, iter::FromIterator};
    lazy_static! {
        static ref HASHMAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m.insert(1, "bar");
            m.insert(2, "baz");
            m
        };
    }
    #[test]
    fn lazy() {
        // First access to `HASHMAP` initializes it
        println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

        // Any further access to `HASHMAP` just returns the computed value
        println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
