#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

macro_rules! co_vec_default {
    () => {
        1
    }
}

struct M<const L: usize> {
    pub entries: [i32; L]
}

impl M<{ co_vec_default!() }> {
    fn f() {}
}

trait G {
    fn g() {}
}

impl G for M<{ co_vec_default!() }> {
    fn g() {}
}

const H_DEFAULT: usize = co_vec_default!();
trait H {}
impl H for M<H_DEFAULT> {}


/*struct U {}
impl U {
    pub fn ini() -> [i32; {co_vec_default()!} ] {
        [0; co_vec_default()! ]
    }
}*/

struct V<const U: usize>
where
    [(); U]:;

trait Tr {}

impl Tr for V<{co_vec_default!()}> {}
