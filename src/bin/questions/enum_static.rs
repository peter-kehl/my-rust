// Can't: static start:String = String::from("start ");
enum E { I32(i32), STRING(String) }
static E_1:E= E::I32(1);
// Can't: static E_ONE:E= E::STRING( String::from("E_ONE") );

fn main() {
    //String::from("a")+ String::from("b"); // not compiling, expecting String + &str
    println!( "{}", String::from("a")+ &String::from("b") );
    //String::from("a")+ &*String::from("b"); //compiling
}