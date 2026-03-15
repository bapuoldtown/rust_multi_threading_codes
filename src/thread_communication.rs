// ============================================================================
// MODULE 2: THREAD COMMUNICATION — Message Passing with Channels
// ============================================================================
//
// 🧠 THE TWO WAYS THREADS COMMUNICATE:
//
//   1. MESSAGE PASSING (this module): Send data through channels
//      → "Don't communicate by sharing memory; share memory by communicating"
//      → Inspired by Go's philosophy and CSP (Communicating Sequential Processes)
//
//   2. SHARED STATE (next module): Multiple threads access the same memory
//      → Requires locks (Mutex, RwLock) to prevent data races
//
// CHANNELS IN RUST:
// ─────────────────
// A channel is like a one-way pipe:
//   - Sender (tx) → puts data IN
//   - Receiver (rx) → takes data OUT
//
// std::sync::mpsc = "Multiple Producer, Single Consumer"
//   - Multiple senders can send to ONE receiver
//   - The receiver processes messages in FIFO order
//
// Think of it like a restaurant:
//   - Multiple waiters (senders/producers) bring orders to the kitchen
//   - One chef (receiver/consumer) processes them one at a time
//
// ================================================================
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn thread_communication(){
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     MODULE 2: MESSAGE PASSING — Channels & Communication    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
 
    // ========================================================================
    // LESSON 1: Basic Channel — Send & Receive
    // ========================================================================
    //
    // mpsc::channel() creates an UNBOUNDED (infinite buffer) channel.
    // It returns a tuple: (Sender<T>, Receiver<T>)
    //
    // Key behaviors:
    //   - send() never blocks (unbounded channel)
    //   - recv() BLOCKS until a message arrives or all senders are dropped
    //   - When all senders are dropped, recv() returns Err(RecvError)
    //
    println!("━━━ Lesson 1: Basic Channel ━━━\n");
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     MODULE 2: MESSAGE PASSING — Channels & Communication    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
 
    // ========================================================================
    // LESSON 1: Basic Channel — Send & Receive
    // ========================================================================
    //
    // mpsc::channel() creates an UNBOUNDED (infinite buffer) channel.
    // It returns a tuple: (Sender<T>, Receiver<T>)
    //
    // Key behaviors:
    //   - send() never blocks (unbounded channel)
    //   - recv() BLOCKS until a message arrives or all senders are dropped
    //   - When all senders are dropped, recv() returns Err(RecvError)
    //
    println!("━━━ Lesson 1: Basic Channel ━━━\n");

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "thread"];
        for message in messages{
            sender.send(message).unwrap();
            println!("Sent message: {}", message);
            thread::sleep(Duration::from_millis(50));
        }
        println!("sender is dropped here");
    });

    //Now poll the messages in receiver
    loop{
        match receiver.recv(){
            Ok(msg) => println!("Received message: {}", msg),
            Err(_) => {
                println!("All senders have been dropped. No more messages.");
                break;
            }
        }
    }

}

pub fn thread_loop_receive_channel() -> Result<Vec<i32>, Box<dyn std::error::Error>>{
    let (sender, receiver) = mpsc::channel();
    let mut result_vector = Vec::new();
    thread::spawn(move || {
        for i in 1..=10{
            sender.send(i).unwrap();
            println!("Sent message: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    //Now loop over the channel to receive messages
    for receiver in receiver{
        println!("Received message: {}", receiver);
        result_vector.push(receiver);
    }
    return Ok(result_vector);
}


pub fn multiple_producers_single_consumer() -> Result<(), Box<dyn std::error::Error>>{
    let (sender, receiver) = mpsc::channel();
    //spwans mulyiple clone of orginal sender give, so you can call a thread and pass each clone to spawn a thread inside the loop
    for i in 1..=10{
        let thread_sender = sender.clone();
        thread::spawn(move || {
            thread_sender.send(i).unwrap();
            println!("Sent message: {}", i);
            thread::sleep(Duration::from_millis(50));
        });
    }
    //drop thhe original sender channel to ensure // IMPORTANT: Drop the original tx!
    // If we don't, the channel never closes because we're holding a sender
    drop(sender);
    // Receive all messages from all producers
    for msg in receiver {
        println!("  📬 {}", msg);
    }
    println!("  All producers done!\n");
    Ok(())

}

pub fn multiple_producers_single_consumer_bounded_channel() -> Result<(), Box<dyn std::error::Error>>{
    // ========================================================================
    // LESSON 4: Bounded Channels — sync_channel()
    // ========================================================================
    //
    // mpsc::sync_channel(N) creates a BOUNDED channel with capacity N.
    //
    // KEY DIFFERENCE:
    //   - channel():      send() NEVER blocks (infinite buffer)
    //   - sync_channel(): send() BLOCKS when buffer is full!
    //
    // WHY use bounded channels?
    //   1. Back-pressure: Slow down producers when consumer can't keep up
    //   2. Memory control: Don't let messages pile up infinitely
    //   3. Flow control: Natural rate limiting
    //
    // sync_channel(0) = "rendezvous channel"
    //   → sender blocks until receiver is ready (direct handoff)
    //   → Useful for synchronization: "I won't proceed until you've received this"
    //
    // Buffer size of 2 — can hold 2 messages before send() blocks
    println!("━━━ Lesson 4: Bounded Channels ━━━\n");
    let (sender, receiver) = mpsc::sync_channel(2);
 
    thread::spawn(move || {
        for i in 0..5 {
            println!("  📤 Sending {}...", i);
            sender.send(i).unwrap(); // Blocks if buffer is full!
            println!("  ✅ Sent {}", i);
        }
    });

    // Slow consumer — the producer will block waiting for space
    for msg in receiver {
        thread::sleep(Duration::from_millis(800));
        println!("  📥 Consumed: {}", msg);
    }

    Ok(())

}

pub fn thread_receive_channel_non_blocking() -> Result<(), Box<dyn std::error::Error>>{
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        sender.send("delayed message").unwrap();
    });

    // Poll without blocking
    for attempt in 1..=5 {
        match receiver.try_recv() {
            Ok(msg) => {
                println!("  ✅ Got message: {} in attempt {}", msg, attempt);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("  ⏳ Attempt {}: No message yet, doing other work...", attempt);
                thread::sleep(Duration::from_millis(80));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("  ❌ Channel disconnected!");
                break;
            }
        }
    }
    println!();
    
    Ok(())

}