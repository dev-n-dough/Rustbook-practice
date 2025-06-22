use std::thread;
use std::time::Duration;

fn main() {
    // simple_thread();
    join_thread();
}

fn join_thread(){
    let handle = thread::spawn( 
        ||{
            for i in 1..10{
                println!("Hi number {i} from spawned thread!");
                thread::sleep(Duration::from_millis(100));
            }
        }
    );

    let handle2 = thread::spawn( 
        ||{
            for i in 1..10{
                println!("Hi number {i} from spawned thread! SECOND");
                thread::sleep(Duration::from_millis(100));
            }
        }
    );

    // handle.join().unwrap();

    for i in 1..5{
        println!("Hi number {i} from main thread!");
        // thread::sleep(Duration::from_millis(1));
    }

    // for i in 1..100000000 {}

    handle.join().unwrap();

    for i in 1..5{
        println!("Hi number {i} from main thread! AGAIN!!");
        // thread::sleep(Duration::from_millis(1));
    }    
}

fn simple_thread(){
    thread::spawn( // a closure:
        ||{
            for i in 1..10{
                println!("Hi number {i} from spawned thread!");
                thread::sleep(Duration::from_millis(200));
            }
        }
    );

    for i in 1..100000000 {}

    for i in 1..5{
        println!("Hi number {i} from main thread!");
        // thread::sleep(Duration::from_millis(1));
    }
}