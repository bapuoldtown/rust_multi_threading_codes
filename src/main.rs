mod basic_thread;
mod thread_communication;
mod shared_state_threading;
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

    match thread_communication::thread_receive_timeout_error(){
        Ok(_) => println!("Channel receive with timeout successful."),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }


    match thread_communication::thread_enum_send_channel(){
        Ok(_) => println!("Channel receive messages with a structred format tagging enum to the message structre."),
        Err(e) => eprintln!("Thread panicked with error: {}", e),
    }
    match shared_state_threading::arc_shared_copy(){
        Ok(_) => println!("Arc sare copy to avoiud repetative clone "),
        Err(e) => eprintln!("Thread panicked with error: {}", e),

    }
    match shared_state_threading::one_resource_at_a_time(){
        Ok(_) => println!("Mutex with Locks to isolate resource while operating on the same resources "),
        Err(e) => eprintln!("Thread panicked with error: {}", e),

    }
    match shared_state_threading::concurrent_likes_scenarios(){
        Ok(_) => println!("Concurrent likes "),
        Err(e) => eprintln!("Thread panicked with error: {}", e),

    }
}
