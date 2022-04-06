fn main() {
    match 1 { // https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html
        n @1 => format!("One {}", n), _ => String::from("Not one")
    }
}