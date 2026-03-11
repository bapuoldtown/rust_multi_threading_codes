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

// ========================================================================
    // LESSON 4: Spawning Multiple Threads & Collecting Results
    // ========================================================================
    //
    // Common pattern: Spawn N threads, collect all results.
    // Each thread does independent work, then we gather everything.
    //
    // This is the foundation of parallel computation!
    //
pub fn thread_joining() -> Result<Vec<usize>, Box<dyn std::error::Error>>{
    let mut handler_vec = vec![];
    for i in 1..=5{
        let handler = thread::spawn(move ||{
            println!("Thread {} is running",i);
            i*2
        });
        handler_vec.push(handler);

    }
    let final_result: Vec<usize> = handler_vec.into_iter().map(|h| h.join().unwrap()).collect();
    return Ok(final_result);

}

pub fn thread_scope_sharing() -> Result<(usize, usize), Box<dyn std::error::Error>>{

// ========================================================================
    // LESSON 6: Scoped Threads (Rust 1.63+) — Borrowing Data Across Threads
    // ========================================================================
    //
    // PROBLEM with regular threads: You must `move` data or use Arc.
    // You CAN'T borrow data from the stack because the thread might outlive it.
    //
    // SOLUTION: std::thread::scope() creates a "scope" where:
    //   - All threads spawned inside are GUARANTEED to finish before scope ends
    //   - Therefore, they CAN borrow local data safely!
    //   - No need for `move`, Arc, or Mutex for read-only access!
    //
    // This is HUGE for performance — no heap allocation needed!
    //
    let source_data = vec![1,2,3,4,5,6,7,8,9,10];
    let chunk_size: usize = source_data.len()/2;
    let (thread_out1, thread_out2) = thread::scope(|s| {
        let handle1 = s.spawn(||{
            let data_sum: &usize = &source_data[..chunk_size].iter().sum();
            return *data_sum;
        });
        let handle2= s.spawn(||{
            let data_sum: &usize = &source_data[chunk_size..].iter().sum();
            return *data_sum;
        });

        return (handle1.join().unwrap(), handle2.join().unwrap());
    });
    return Ok((thread_out1, thread_out2))
}

pub fn thread_scope_sharing_with_mut() -> Result<(usize, usize), Box<dyn std::error::Error>>{
    // ========================================================================
    // LESSON 8: Send and Sync Traits — The Compile-Time Safety Net
    // ========================================================================
    //
    // These marker traits are WHY Rust threading is safe:
    //
    // Send: "This type can be transferred to another thread"
    //   - Most types are Send (i32, String, Vec, etc.)
    //   - NOT Send: Rc<T> (reference counting isn't thread-safe)
    //   - NOT Send: raw pointers (no guarantees)
    //
    // Sync: "This type can be shared (via &T) between threads"
    //   - T is Sync if &T is Send
    //   - NOT Sync: Cell<T>, RefCell<T> (interior mutability isn't thread-safe)
    //   - Mutex<T> IS Sync (it provides thread-safe interior mutability)
    //
    // The compiler checks these automatically!
    // If you try to send a non-Send type to a thread, it WON'T COMPILE.
    //
    println!("━━━ Lesson 8: Send & Sync Traits ━━━\n");

    // This works because i32 is Send + Sync
    let number = 42;
    let handle = thread::spawn(move || {
        println!("  i32 is Send: {}", number);
    });
    handle.join().unwrap();

    // This works because String is Send + Sync
    let text = String::from("hello threads");
    let handle = thread::spawn(move || {
        println!("  String is Send: {}", text);
    });
    handle.join().unwrap();

    // This would NOT compile:
    // let rc = std::rc::Rc::new(42);
    // thread::spawn(move || {
    //     println!("{}", rc); // ERROR: Rc<i32> is not Send!
    // });
    // Use Arc instead of Rc for thread-safe reference counting
    Ok((100 as usize, 200 as usize))
}