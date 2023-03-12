use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    let s = String::from("Hello");

    // create a thread that will be joined
    thread::scope(|_|{
        println!("Length: {}", s.len());
        thread::sleep(Duration::from_millis(1000));
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
