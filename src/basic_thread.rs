// ============================================================================
// MODULE 1: THREAD BASICS — From Absolute Zero to Thread Mastery
// ============================================================================
//
// CORE CONCEPTS:
//
// WHAT IS A THREAD?
// -----------------
// A thread is the smallest unit of execution that the OS can schedule.
// Think of it like a worker in a factory:
//   - The PROCESS is the factory (it has memory, resources, file handles)
//   - Each THREAD is a worker inside that factory
//   - All workers SHARE the factory floor (shared memory space)
//   - Each worker has their OWN notebook (stack) to track what they're doing
//
// CONCURRENCY vs PARALLELISM:
// ---------------------------
// Concurrency = Multiple tasks making progress (could be interleaved on 1 CPU)
//   → Like a chef switching between chopping and stirring
//
// Parallelism = Multiple tasks running SIMULTANEOUSLY on different CPUs
//   → Like two chefs each doing their own dish at the same time
//
// Rust gives you BOTH. Threads enable parallelism when you have multiple CPU
// cores, and concurrency even on a single core (via OS scheduling).
//
// WHY RUST IS SPECIAL FOR THREADING:
// -----------------------------------
// Most languages let you create data races at runtime (C/C++, Java, Python).
// Rust prevents data races at COMPILE TIME through:
//   1. Ownership system — one owner at a time
//   2. Send trait — can this type be sent to another thread?
//   3. Sync trait — can this type be shared between threads?
//
// If your code compiles, it's FREE of data races. This is Rust's
// "fearless concurrency" guarantee.
//
// ============================================================================

use std::thread;
use std::time::Duration;

pub fn thread_basics() -> Result<i32, Box<dyn std::error::Error>>{
    // ========================================================================
    // LESSON 1: Spawning Your First Thread
    // ========================================================================
    //
    // std::thread::spawn() creates a new OS thread.
    // It takes a closure (anonymous function) that the thread will execute.
    //
    // IMPORTANT: The main thread does NOT wait for spawned threads by default!
    // If main() ends, ALL spawned threads are killed immediately.
    //
    let handle = thread::spawn(|| { for i in 1..=5 {
        println!("The thread counter first is {}", i);
        thread::sleep(Duration::from_millis(20));
    }
    42});

    //Main thread sleeps
    for i in 1..=3{
        println!("The thread counter second is {}", i);
        thread::sleep(Duration::from_millis(50));
    }
    let result = handle.join().unwrap();
    return Ok(result);

}

pub fn thread_move_scenarios() -> Result<String, Box<dyn std::error::Error>>{
    // ========================================================================
    // LESSON 2: The `move` Keyword — Ownership Transfer to Threads
    // ========================================================================
    //
    // Problem: Threads might outlive the data they reference!
    //
    // WRONG (won't compile):
    //   let name = String::from("Guri");
    //   thread::spawn(|| {
    //       println!("{}", name); // ERROR: `name` might be dropped before thread uses it!
    //   });
    //
    // SOLUTION: Use `move` to TRANSFER ownership into the thread's closure.
    // After move, the original variable is no longer accessible.
    //
    println!("Demonstrating move scenarios:");
    let name1= String::from("Guri");
    let name2 = String::from("Prasad");
    let result_thread = thread::spawn(move || {
        let result = format!("Hello, {} and {}", name1, name2);
        thread::sleep(Duration::from_millis(100));
        return result;

    });
    thread::sleep(Duration::from_millis(50));
    return Ok(result_thread.join().unwrap());

}

pub fn thread_builder_pattern() -> Result<String, Box<dyn std::error::Error>>{
    // ========================================================================
    // LESSON 3: Thread Builder — Customizing Thread Properties
    // ========================================================================
    //
    // thread::Builder lets you set:
    //   - name: Useful for debugging (shows in panic messages & profilers)
    //   - stack_size: Override default stack size (usually 8MB on Linux)
    //
    let thread_builder  = thread::Builder::new().name("Custom thread".to_string()).stack_size(4*1024*1024);
    let handle = thread_builder.spawn(|| {
        let current_thread = thread::current();
        return format!("Running in thread name: {} and current id is {:?}", current_thread.name().unwrap_or("Unnamed"), current_thread.id())
    })?;
    
    //let handle = handle?;
    return Ok(handle.join().unwrap());
}

