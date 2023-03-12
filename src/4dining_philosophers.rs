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

        // this solution seems to work but I'm not sure if it is correct?
        // NB: this differs from the official solution which don't use try
        // and uses a resource hierarchy to avoid deadlock (which might leave to starvation
        // of a philosopher)
        // see https://google.github.io/comprehensive-rust/exercises/day-4/solutions-morning.html
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
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

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
