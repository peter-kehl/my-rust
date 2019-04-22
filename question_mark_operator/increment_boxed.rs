type IncResult<T>= Result<Box<T>, &'static str>;
trait Inc {
    fn inc(self) -> IncResult<Self>;//Result<Box<Self>, &'static str>;
}

type Inc32Result= IncResult<i32>;

impl Inc for i32 {
    fn inc(self) -> Inc32Result {
        if self<std::i32::MAX {
            Ok(Box::new(self+1))
        }
        else {
            Err("Overflown")
        }
    }
}

fn inc_ok() -> Inc32Result {
    Box::new(1).inc()?.inc()
}

fn inc_overflow() -> Inc32Result {
    Box::new( std::i32::MAX ).inc()?.inc()?.inc()?.inc()?.inc()?.inc()
}

fn main() {
    println!( "{:?}", inc_ok());
    println!( "{:?}", inc_overflow());
}
