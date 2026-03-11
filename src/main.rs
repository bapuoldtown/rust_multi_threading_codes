mod basic_thread;
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
}
