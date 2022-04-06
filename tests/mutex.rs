use std::sync::Mutex;
use std::thread;

#[test]
fn mutex() {
    let max = Mutex::new(0u16);
    let max_ref = &max;
    //thread::spawn( || max_ref );
}
