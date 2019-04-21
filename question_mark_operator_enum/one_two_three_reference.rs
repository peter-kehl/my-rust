trait Inc {
    fn inc<'scope>(&self) -> Result<&'scope Self, &'scope str>; //or 'static str instead of 'scope str
}

#[derive(Debug)]
enum OneTwoThree {ONE, TWO, THREE}
type OneTwoThreeResult<'scope>= Result<&'scope OneTwoThree, &'static str>;

impl Inc for OneTwoThree {
    fn inc<'scope>(&self) -> OneTwoThreeResult<'scope> {
        match self {
            OneTwoThree::ONE => Ok(&OneTwoThree::TWO),
            OneTwoThree::TWO => Ok(&OneTwoThree::THREE),
            OneTwoThree::THREE => Err("Overflown 3")
        }
    }
}

fn inc_ok<'scope>() -> OneTwoThreeResult<'scope> {
    OneTwoThree::ONE.inc()?.inc()
}

fn inc_overflow<'scope>() -> OneTwoThreeResult<'scope> {
    OneTwoThree::ONE.inc()?.inc()?.inc()?.inc()?.inc()?.inc()
}

fn main() {
    println!( "{:?}", inc_ok());
    println!( "{:?}", inc_overflow());
}
