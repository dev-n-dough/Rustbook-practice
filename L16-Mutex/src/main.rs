use std::sync::{Mutex,Arc};
use std::thread;

fn main() {
    // simple_mutex();
    multiple_owners_multithreading();
}

fn simple_mutex(){
    let num = 5;
    let m = Mutex::new(num); // clearly the data inside is immutable

    {
        let mut num = m.lock().unwrap(); // but we can make it mutable on command -> this is similar to RefCell<T> !!!
        *num=6;
    }

    println!("m is {m:?}");
}

fn multiple_owners_multithreading(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num+= 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("counter is finally = {}", *counter.lock().unwrap());
}
