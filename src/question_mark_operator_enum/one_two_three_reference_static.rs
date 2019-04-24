type IncResult<T>= Result<&'static T, &'static str>;
trait Inc {
    fn inc(&self) -> IncResult<Self>;
}

#[derive(Debug)]
enum OneTwoThree {ONE, TWO, THREE}
type OneTwoThreeResult= IncResult<OneTwoThree>;

impl Inc for OneTwoThree {
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
