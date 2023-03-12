use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...

        match (self.left_fork.try_lock(), self.right_fork.try_lock()) {
            (Ok(_), Ok(_)) => {
                println!("{} is eating...", &self.name);
                thread::sleep(Duration::from_millis(10));
            }
            _ => {
                //println!("{} is hungry...", &self.name);
            }
        }
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let forks = [
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
    ];

    let (tx, rx) = mpsc::channel();

    // Create philosophers
    let mut philosophers: Vec<Philosopher> = Vec::new();
    for (i, p) in PHILOSOPHERS.iter().enumerate() {
        let philosopher = Philosopher {
            name: p.to_string(),
            left_fork: forks[i].clone(),
            right_fork: forks[(i + 1) % forks.len()].clone(),
            thoughts: tx.clone(),
        };
        philosophers.push(philosopher);
    }

    // Make them think and eat
    for p in philosophers {
        let philosopher = p;
        thread::spawn(move || {
            loop {
                philosopher.think();
                philosopher.eat();
            }
        });
    }

    // Output their thoughts
    for msg in rx.iter() {
        println!("{}", msg);
    }
}
