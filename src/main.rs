mod basic_thread;
mod thread_communication;
fn main() {
    println!("Hello, world!");
    //running the basic code first
    match basic_thread::thread_basics(){
        Ok(result) => println!("The thread returned: {}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

    //move code
    match basic_thread::thread_move_scenarios(){
        Ok(result) => println!("The thread returned: {:?}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

    match basic_thread::thread_builder_pattern(){
        Ok(result) => println!("The thread returned: {}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

    match basic_thread::thread_joining(){
        Ok(result) => println!("The thread returned: {:?}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

    match basic_thread::thread_scope_sharing(){
        Ok(result) => println!("The thread returned: {:?}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }
    match basic_thread::thread_scope_sharing_with_mut(){
        Ok(result) => println!("The thread returned: {:?}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),   
    }


    //Chapter two thread communication
    thread_communication::thread_communication();
    match thread_communication::thread_loop_receive_channel(){
        Ok(result) => println!("The thread returned: {:?}", result),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }
    match thread_communication::multiple_producers_single_consumer(){
        Ok(_) => println!("Multiple producers sent messages successfully."),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

    match thread_communication::multiple_producers_single_consumer_bounded_channel(){
        Ok(_) => println!("Multiple producers sent messages successfully on bounded channel."),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

    match thread_communication::thread_receive_channel_non_blocking(){
        Ok(_) => println!("Non-blocking channel receive successful."),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }

}
