trait Inc {
    fn inc<'scope>(&self) -> Result<&'scope Self, &'scope str>; //or 'static str instead of 'scope str
}

#[derive(Debug)]
enum OneTwoThree {ONE, TWO, THREE, MORE(i32)}
type OneTwoThreeResult<'scope>= Result<&'scope OneTwoThree, &'static str>;

impl Inc for OneTwoThree {
    fn inc<'scope>(&self) -> OneTwoThreeResult<'scope> {
        match self {
            OneTwoThree::ONE => Ok(&OneTwoThree::TWO),
            OneTwoThree::TWO => Ok(&OneTwoThree::THREE),
            OneTwoThree::THREE => Err("Overflown 3"), // intentional separation between THREE and MORE
            OneTwoThree::MORE(i) => if i<&std::i32::MAX {
                Ok( &OneTwoThree::MORE(i+1) )
            }
            else {
                Err("Overflown i32 limit")
            }
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
