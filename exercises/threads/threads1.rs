// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.

use std::thread::{self, JoinHandle};
use std::time::Duration;


fn main() {

    let mut handles = vec![];
    for i in 0..10 {
            let u = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        });
        handles.push(u);
    }
    // 理论上到这里，handles里面有10个value了

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        match handle.join() {
            Ok(_) => {
                completed_threads+=1;
            },
            Err(_) => {}
        }
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
}
