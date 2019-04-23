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

macro_rules! hashy {
    // Can't have a colon : as a separator instead of =>.
    ( $($key:expr => $value:expr),+ ) => {{{
        /*hashy!( $key, $value);
        hashy!( $($key2, $value2),+ );*/
        let mut map= HashMap::new();
        $(
            map.insert( $key, $value );
        )*
        map
    }}}
}

fn main() {
    use_say_hello();
    //println!( "{:?}", hashy!(1, "one", 2, "two") );
    let mut map= hashy! {1 => "one", 2 => "two"};
    map.insert( 3, "three");
    println!( "{:?}",  map );
}