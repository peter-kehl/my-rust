// For https://doc.rust-lang.org/stable/rust-by-example/flow_control/if_let.html#if-let

fn main() {
    struct Two(i32,i32);
    let t=Two(1,2);
    if let Two(1,i)=t {
        println!( "Matched {}", i); // ==> 2
    }
    else {
        panic!("Not matched");
    };

    struct V(i32);
    let v=V(1);

    if let V(x)=v {
        println!( "{}", x ); // ==> 1
    }
    else {
        panic!( "Not matched." );
    }
    // However, the previous `if let` generates "irrefutable if-let pattern" warning,
    // because there's no possiblity of executing the `else` branch.
    // Hence, you can destructure instead:
    let V(x)=v;
    println!( "{}", x ); // ==> 1

}