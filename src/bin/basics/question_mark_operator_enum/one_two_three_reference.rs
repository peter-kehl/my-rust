type IncResult<'scope, T>= Result<&'scope T, &'static str>;
trait Inc<'scope> {
    fn inc(&self) -> IncResult<'scope, Self>;//Result<&'scope Self, &'static str>;
}

#[derive(Debug)]
enum OneTwoThree {ONE, TWO, THREE}
type OneTwoThreeResult= IncResult<'static, OneTwoThree>;

impl Inc<'static> for OneTwoThree {
    fn inc(&self) -> OneTwoThreeResult {
        match self {
            OneTwoThree::ONE => Ok(&OneTwoThree::TWO),
            OneTwoThree::TWO => Ok(&OneTwoThree::THREE),
            OneTwoThree::THREE => Err("Overflown 3")
        }
    }
}

fn inc_ok() -> OneTwoThreeResult {
    OneTwoThree::ONE.inc()?.inc()
}

fn inc_overflow() -> OneTwoThreeResult {
    OneTwoThree::ONE.inc()?.inc()?.inc()?.inc()?.inc()?.inc()
}

fn main() {
    println!( "{:?}", inc_ok());
    println!( "{:?}", inc_overflow());
}
