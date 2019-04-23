type IncResult<T>= Result<T, &'static str>;
trait Inc {
    fn inc(self) -> IncResult<Self>
    where Self:Sized; // <<<<<<<<<<<<<<<<<<
}

type Inc32Result= IncResult<i32>;

impl Inc for i32 {
    fn inc(self) -> Inc32Result {
        if self<std::i32::MAX {
            Ok(self+1)
        }
        else {
            Err("Overflown")
        }
    }
}

fn inc_ok() -> Inc32Result {
    1.inc()?.inc()
}

fn inc_overflow() -> Inc32Result {
    std::i32::MAX.inc()?.inc()?.inc()?.inc()?.inc()?.inc()
}

fn main() {
    println!( "{:?}", inc_ok());
    println!( "{:?}", inc_overflow());
}
