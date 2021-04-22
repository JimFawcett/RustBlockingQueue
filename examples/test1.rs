/////////////////////////////////////////////////////////////
// rust_blocking_queue::test1.rs - demo blocking queue     //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 26 May 2020  //
/////////////////////////////////////////////////////////////
/*
   This is a good demonstration of the way BlockingQueue
   will be used in an application.
*/
use std::io::*;
use std::sync::*;
use std::thread;
use rust_blocking_queue::{BlockingQueue};
use std::time::Duration;

fn main() {

    print!("\n  Demonstrate queue shared between threads");
    print!("\n ==========================================");

    test();
    print!("\n\n  That's all Folks!\n");
}

/*-- simple test of BlockingQueue --*/
fn test() {

    let share = Arc::new(BlockingQueue::<String>::new());
    let share1 = Arc::clone(&share);
    let share2 = Arc::clone(&share);

    let flush = || { let _ = std::io::stdout().flush(); };

    /*-- child thread dequeues messages --*/
    let handle = thread::spawn(move || {
        print!("\n  child thread started");
        flush();
        let dur = Duration::from_millis(50);
        loop {
            let t = share1.de_q();
            print!("\n  dequeued {} on child thread", t);
            flush();
            if &t == "quit" {
                break;
            }
            thread::sleep(dur);
        }
        print!("\n  thread shutting down");
        flush();
    });

    /*-- main thread enqueues messages --*/
    let dur = Duration::from_millis(20);
    for i in 0..5 {
        let msg = format!("msg #{}", i.to_string());
        print!("\n  enqueued {:?} on main thread", msg);
        flush();
        share2.en_q(msg);
        thread::sleep(dur);
    }
    /*-- shut down child thread --*/
    print!("\n  enqueued {:?} on main thread", "quit");
    flush();
    share2.en_q("quit".to_string());

    /*-- child thread must complete before exiting --*/
    print!("\n  waiting for child thread to stop");
    flush();
    let _ = handle.join();

    print!("\n  queue length = {}", share2.len());
}
