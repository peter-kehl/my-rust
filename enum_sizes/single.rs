enum Single{SINGLE}

fn main() {
    println!("{}", std::mem::size_of_val(&Single::SINGLE) ); // -> 0 bytes
}
