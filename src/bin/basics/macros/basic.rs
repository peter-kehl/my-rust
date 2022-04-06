use std::collections::HashMap;

macro_rules! say_hello {
    () => (
        println!("Hello!")
    );
    ($expr:expr) => (
        { // <--- must be in a block, if more than 1 statement
            println!("Evaluating {}", stringify!($expr));
            $expr
        }
    )
}

fn use_say_hello() {
    say_hello!();
    println!( "{}", say_hello!(1+2) );
}

#[macro_export]
macro_rules! hashy {
    // Can't have a colon : as a separator instead of =>.
    ( $($key:expr => $value:expr),+ ) => {{{
        let mut map= HashMap::new();
        $(
            map.insert( $key, $value );
        )*
        map
    }}}
}

fn use_hashy() {
    let mut map= hashy! {1 => "one", 2 => "two"};
    map.insert( 3, "three");
    println!( "{:?}",  map );
}

#[macro_export]
macro_rules! structy {
    ( $($key:ident => $value:expr),+ ) => {{{
        let mut map= HashMap::new();
        $(
            map.insert( stringify!($key), $value );
        )*
        map
    }}}
}


fn main() {
    use_say_hello();
    use_hashy();
}