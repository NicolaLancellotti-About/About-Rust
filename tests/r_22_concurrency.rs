#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::*;
use std::thread;
use std::time::Duration;

/*
    Send: Types that can be transferred across thread boundaries.
    Sync: Types for which it is safe to share references between threads.
*/

#[test]
fn thread() {
    let a_box: Box<i32> = Box::new(10);

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("value: {}", a_box);
    });

    let r = handle.join();
}

#[test]
fn message_passing() {
    enum Message {
        Item(Box<i32>),
    }

    //  Multiple producer, single consumer.
    let (sender1, receiver) = mpsc::channel();
    let receiver1 = Arc::new(Mutex::new(receiver));
    let receiver2 = receiver1.clone();
    let sender2 = sender1.clone();

    // producer 1
    let producer1 = thread::spawn(move || {
        for i in 0..2 {
            thread::sleep(Duration::from_millis(300));
            println!("P1 - {}", i);
            let data = Message::Item(Box::new(i));
            sender1.send(data).unwrap();
        }
    });

    // producer 2
    let producer2 = thread::spawn(move || {
        for i in 2..4 {
            thread::sleep(Duration::from_millis(300));
            println!("P2 - {}", i);
            let data = Message::Item(Box::new(i));
            sender2.send(data).unwrap();
        }
    });

    // consumer 1
    let consumer1 = thread::spawn(move || {
        loop {
            match receiver1.lock().unwrap().recv() {
                Ok(Message::Item(value)) => println!("C1 - {}", value),
                Err(_) => break, // Channel closed,
            }
        }
    });

    // consumer 2
    let consumer2 = thread::spawn(move || {
        loop {
            match receiver2.lock().unwrap().recv() {
                Ok(Message::Item(value)) => println!("C2 - {}", value),
                Err(_) => break, // Channel closed,
            }
        }
    });

    let r = producer1.join();
    let r = producer2.join();
    let r = consumer1.join();
    let r = consumer2.join();
}

#[test]
fn condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        thread::sleep(::std::time::Duration::from_secs(2));
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    while !*started {
        started = cvar.wait(started).unwrap();
    }
}
