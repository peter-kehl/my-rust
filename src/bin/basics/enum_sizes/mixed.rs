#[derive(PartialEq)]
enum Mixed { SINGLE, I32(i32) }

fn mixed(value:i32)->Mixed {
    if value>0 {
        Mixed::I32(value)
    } else {
        Mixed::SINGLE
    }
}

fn main() {
    let single=mixed(0);
    assert!(single==Mixed::SINGLE);
    let one=mixed(1);
    println!("{} {}", std::mem::size_of_val(&single), std::mem::size_of_val(&one) ); // -> 8 and 8 bytes

    
}
