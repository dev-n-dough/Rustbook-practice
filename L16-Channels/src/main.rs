use std::sync::mpsc;
use std::thread;

fn main() {
    // single_receiver();
    bounded_channel();
    // bounded_zero_capacity(); 
}

use crossbeam_channel::bounded;

fn bounded_zero_capacity(){
    let (s,r) = bounded(0);

    thread::spawn(
        move || {
            s.send("Hi!").unwrap();
            println!("From alt thread");
        }
    );

    // println!("{}" , r.recv().unwrap());
}

fn bounded_channel(){
     // Create a bounded channel with capacity 5.
     let (s, r) = bounded(5);

     // Fill up the channel with 5 messages.
     for i in 0..5 {
         s.send(i).unwrap();
     }
 
    // This send call will block because the channel is full.
    
    // Spawn a thread to receive messages after some delay.
    let receiver_thread = thread::spawn(move || {
        for _ in 0..5 {
            println!("Received: {}", r.recv().unwrap());
        }
    });

    
    println!("Attempting to send...");
    s.send(5).unwrap(); // Blocks until a message is received.
    println!("Message sent!");

    receiver_thread.join().unwrap();

}

fn single_receiver(){

    let (tx,rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi!");
        tx.send(val).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Received : {msg}");
}