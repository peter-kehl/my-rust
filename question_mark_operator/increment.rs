trait Inc {
    // Following is cheating - without dynamically sized types ("unsized" types)
    fn inc(self) -> Result<i32/*Self*/, &'static str>;
}

type IncResult= Result<i32, &'static str>;

impl Inc for i32 {
    fn inc(self) -> IncResult {
        if self<std::i32::MAX {
            Ok(self+1)
        }
        else {
            Err("Overflown")
        }
    }
}

fn inc_ok() -> IncResult {
    1.inc()?.inc()
}

fn inc_overflow() -> IncResult {
    std::i32::MAX.inc()?.inc()?.inc()?.inc()?.inc()?.inc()
}

fn main() {
    println!( "{:?}", inc_ok());
    println!( "{:?}", inc_overflow());
}
