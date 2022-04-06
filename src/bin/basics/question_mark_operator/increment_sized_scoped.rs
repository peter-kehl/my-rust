type IncResult<'a, T>= Result<T, &'a str>;
trait Inc {
    fn inc<'a>(self) -> IncResult<'a, Self>
    where Self:Sized; // <<<<<<<<<<<<<<<<<<
}

type Inc32Result<'a>= IncResult<'a, i32>;

impl Inc for i32 {
    fn inc<'a>(self) -> Inc32Result<'a> {
        if self<std::i32::MAX {
            Ok(self+1)
        }
        else {
            Err("Overflown")
        }
    }
}

fn inc_ok<'a>() -> Inc32Result<'a> {
    1.inc()?.inc()
}

fn inc_overflow<'a>() -> Inc32Result<'a> {
    std::i32::MAX.inc()?.inc()?.inc()?.inc()?.inc()?.inc()
}

fn main() {
    println!( "{:?}", inc_ok());
    println!( "{:?}", inc_overflow());
}
