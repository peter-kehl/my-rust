#[derive(Debug)]
struct MyStruct(i32);

/*impl MyStruct {
    fn new(value: i32) -> MyStruct {
        println!("MyStruct::new( {} )", value);
        MyStruct(value)
    }
}*/

impl Copy for MyStruct { }

// Since we want Copy, we have to implement Clone, too
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        println!( "Clone of # {} will have value # {}.", self.0, self.0+1);
        // the most efficient is to return: *self
        MyStruct( self.0+1 )
        //MyStruct( String::from(self.0))
        // TODO Box(&str)
    }
}

fn create_and_return(value: i32) -> MyStruct {
    MyStruct(value)
}

fn main() {
    let base= MyStruct(0);
    //let base= MyStruct::new(0);
    let _first= base;
    let second= base;
    println!("copied {:?}", second );
    println!("cloned {:?}", second.clone() );

    let created_and_returned = create_and_return(5);
    println!("create_and_return() -> # {:?}.", created_and_returned);

    let boxed= Box::new( base );
    println!("boxed {:?}", boxed );
}