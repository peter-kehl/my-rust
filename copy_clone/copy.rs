#[derive(Debug)]
struct MyStruct(i32);

/*impl MyStruct {
    fn new(value: i32) -> MyStruct {
        println!("MyStruct::new( {} )", value);
        MyStruct(value)
    }
}*/

impl Copy for MyStruct { }

impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        println!( "Clone of # {}", self.0);
        //*self
        MyStruct( self.0+1 )
        //MyStruct( String::from(self.0))
        // TODO Box(&str)
    }
}

fn main() {
    let base= MyStruct(0);
    //let base= MyStruct::new(0);
    let _first= base;
    let second= base;
    println!("copied {:?}", second );
    let boxed= Box::new( base );
    println!("boxed {:?}", boxed );
}