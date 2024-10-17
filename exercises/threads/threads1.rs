// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));  // 模拟线程工作 250ms
            println!("thread {} is complete", i);
            start.elapsed().as_millis()  // 返回经过的时间
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // 使用 handle.join() 等待线程完成并获取返回值
        let result = handle.join().unwrap();  // join 会返回线程的返回值
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}

