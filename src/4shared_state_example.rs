use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));
    let v2 = v.clone();

    let handle = thread::spawn(move || {
        println!("Thread taking lock");
        let mut guard = v2.lock().unwrap();
        guard.push(10);
    });
    println!("Main taking lock");

    {
        let mut guard = v.lock().unwrap();
        guard.push(1000);
    } // lock is released here, without scoping we would deadlock!

    handle.join().unwrap();
    println!("v: {v:?}");
}
