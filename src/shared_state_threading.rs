use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

pub fn arc_shared_copy() -> Result<(), Box<dyn std::error::Error>> {
    //stop cloning of data scenarios when we want to share data across treads
    println!("\n");
    println!("  ╔═══════════════════════════════════════════════════════════╗");
    println!("  ║      SHARED STATE — THE FUN WAY                          ║");
    println!("  ║  Real businesses. Real shared data. Real solutions.      ║");
    println!("  ╚═══════════════════════════════════════════════════════════╝");
    println!("\n");

    // ========================================================================
    // 🍔 SCENARIO 1: McDONALD'S MENU — SHARED READ-ONLY DATA
    // ========================================================================
    //
    // Every cashier (thread) needs to see the menu (data).
    // The menu doesn't change during the shift.
    // Do we PHOTOCOPY the menu for each cashier? NO! That wastes paper!
    // We put ONE menu on the wall and everyone LOOKS at it.
    //
    // clone() = photocopy the menu (wasteful!)
    // Arc = one menu on the wall, everyone shares it (efficient!)
    //
    println!("  ┌─────────────────────────────────────────────────────────┐");
    println!("  │  🍔 SCENARIO 1: McDonald's Menu Board                  │");
    println!("  │  Problem: 8 cashiers need the menu. Copy for each?     │");
    println!("  │  Solution: Arc = ONE menu board, everyone LOOKS at it  │");
    println!("  └─────────────────────────────────────────────────────────┘\n");
    let menu = Arc::new(vec![
        ("Big Mac", 5.99),
        ("McChicken", 4.49),
        ("Quarter Pounder", 6.49),
        ("Filet-O-Fish", 5.29),
        ("10pc McNuggets", 5.99),
    ]);

    println!("  📋 Menu posted! {} items", menu.len());
    println!("  📋 Ref count: {} (just the original)\n", Arc::strong_count(&menu));
    let mut handles = vec![];
    let cashiers = ["Alice", "Bob", "Carol", "Dave", "Eve", "Frank", "Grace", "Hank"];
    for (i, &cashier) in cashiers.iter().enumerate(){
        let menu_clone = Arc::clone(&menu);// NOT a copy! Just a pointer!
        handles.push(thread::spawn(move || {
            let item = &menu_clone[i % menu_clone.len()];
            println!("  🧑‍💼 {}: \"That'll be ${:.2} for a {}!\"", cashier, item.1, item.0);
        }));

    }

    println!("  📋 Ref count with all cashiers: {}", Arc::strong_count(&menu));
    for h in handles {h.join().unwrap()}

    return Ok(())

}

pub fn one_resource_at_a_time() -> Result<(), Box<dyn std::error::Error>>{
    // ========================================================================
    // 🏦 SCENARIO 2: BANK VAULT — ONLY ONE PERSON INSIDE AT A TIME
    // ========================================================================
    //
    // A bank vault has ONE door with a lock.
    // Only ONE person can be inside at a time.
    // Others must WAIT in line until the person leaves.
    // When they leave, the door automatically locks behind them.
    //
    // This is EXACTLY what Mutex does:
    //   .lock()  = enter the vault (blocks if someone's inside)
    //   guard    = you're inside, do your thing
    //   drop     = leave the vault (door auto-locks behind you)
    //
    println!("  ┌─────────────────────────────────────────────────────────┐");
    println!("  │  🏦 SCENARIO 2: Bank Vault — One Person at a Time      │");
    println!("  │  Problem: Two people modifying cash simultaneously     │");
    println!("  │  Solution: Mutex = vault door. One person at a time.   │");
    println!("  └─────────────────────────────────────────────────────────┘\n");
 
    let vault = Arc::new(Mutex::new(100_000.00_f64)); // $100K in the vault
    let start = Instant::now();
    let mut handles = vec![];
    let transactions = [
        ("🧑‍💼 Teller Amy",    "deposit",  25_000.0),
        ("🧑‍💼 Teller Ben",    "withdraw", 50_000.0),
        ("🧑‍💼 Teller Clara",  "deposit",  30_000.0),
        ("🧑‍💼 Teller Dave",   "withdraw", 15_000.0),
    ];
    for (teller, op, amount) in transactions{
        let vault_clone = Arc::clone(&vault);
        let start_counter = start;
        handles.push(thread::spawn(move || {
            println!("  [{:>4}ms] {} waiting to enter vault... 🚪",start_counter.elapsed().as_millis(), teller);
            let mut balance = vault_clone.lock().unwrap(); // 🔐 Enter vault
            println!("  [{:>4}ms] {} INSIDE the vault! Balance: ${:.2}",start_counter.elapsed().as_millis(), teller, *balance);
            thread::sleep(Duration::from_millis(100)); // Counting money...
            match op {
                "deposit" => {
                    *balance += amount;
                    println!("  [{:>4}ms] {} deposited ${:.2} → New balance: ${:.2} 💵",
                        start.elapsed().as_millis(), teller, amount, *balance);
                }
                _ => {
                    *balance -= amount;
                    println!("  [{:>4}ms] {} withdrew ${:.2} → New balance: ${:.2} 💰",
                        start.elapsed().as_millis(), teller, amount, *balance);
                }
            }
            // 🔓 Vault door auto-locks when teller leaves (guard dropped)
            println!("  [{:>4}ms] {} LEFT the vault 🔓", start.elapsed().as_millis(), teller);

        }));


    }
    for h in handles { h.join().unwrap(); }

    Ok(())

}


pub fn concurrent_likes_scenarios() -> Result<(), Box<dyn std::error::Error>>{
    // ========================================================================
    // 📸 SCENARIO 3: INSTAGRAM POST — MILLIONS OF CONCURRENT LIKES
    // ========================================================================
    //
    // Beyoncé posts a photo. 10 million people hit "like" simultaneously.
    // The like counter is ONE number shared across all users.
    // Arc<Mutex<u64>> protects it.
    //
    println!("  ┌─────────────────────────────────────────────────────────┐");
    println!("  │  📸 SCENARIO 3: Instagram — Beyoncé's Viral Post       │");
    println!("  │  Problem: 10 million likes at once on one counter      │");
    println!("  │  Solution: Arc<Mutex<u64>> = thread-safe like counter  │");
    println!("  └─────────────────────────────────────────────────────────┘\n");

    let like_count = Arc::new(Mutex::new(0u64));
    let target_likes = 100_000u64;  // Simulating (scaled down)
    let start = Instant::now();
    thread::scope(|s| {
        for server_id in 0..8{
            let likes = Arc::clone(&like_count);
            s.spawn(move || {
                for _ in 0..(target_likes/8) {
                    *likes.lock().unwrap() += 1;

                }
            });
        }
    });
    let final_likes = *like_count.lock().unwrap();
    println!("\n  📸 Beyoncé's post: ❤️  {} likes in {:?}!", final_likes, start.elapsed());
    Ok(())

}

pub fn resource_locking_good_patterns() -> Result<(), Box<dyn std::error::Error>>{
    // ========================================================================
    // 🎰 SCENARIO 4: CASINO — HOW LONG YOU HOLD THE LOCK MATTERS!
    // ========================================================================
    //
    // Imagine a casino with ONE ATM machine.
    // Player A walks up, checks balance, thinks about it, counts bills,
    // calls their spouse, THEN makes a withdrawal. 10 minutes!
    // Everyone in line is FURIOUS! 😡
    //
    // Player B walks up, knows exactly what they want, withdraws, leaves.
    // 30 seconds! Line moves fast! 😊
    //
    // MORAL: Hold the lock for the SHORTEST TIME POSSIBLE!
    //
    println!("  ┌─────────────────────────────────────────────────────────┐");
    println!("  │  🎰 SCENARIO 4: Casino ATM — Hold the Lock Briefly!    │");
    println!("  │  Problem: One player hogs the ATM for 10 minutes       │");
    println!("  │  Solution: Get in, do your thing, get out FAST         │");
    println!("  └─────────────────────────────────────────────────────────┘\n");

    let balance = Arc::new(Mutex::new(10_000.0_f64));
    // ✅ GOOD: Player decides FIRST, then uses ATM quickly
    let start = Instant::now();
    thread::scope(|s|{
        for pid in 0..4{
            let atm_clone = Arc::clone(&balance); //get clone of the shared data
            let start = start;
            s.spawn(move || {
                // "Think" BEFORE approaching ATM (no lock held!)
                thread::sleep(Duration::from_millis(100)); //b;ockis here not after locking the resource that would create a lot of latency
                let withdrawal = 100.0; // Decision made!
                {
                    let mut balance = atm_clone.lock().unwrap();
                    *balance -= withdrawal;

                }
                println!("  [{:>4}ms] ✅ Player {}: Quick $100 withdrawal!",
                    start.elapsed().as_millis(), pid);

            });

        }
    });
    let good_time = start.elapsed();
    println!("  ✅ Quick in-and-out: {:?} (all 4 done in parallel!)", good_time);
    println!("  💡 Think OUTSIDE the lock, act INSIDE the lock\n\n");
    Ok(())
}

pub fn concurrent_read_single_writer() -> Result<(), Box<dyn std::error::Error>>{
    // ========================================================================
    // 📚 SCENARIO 5: LIBRARY CATALOG — MANY READERS, ONE UPDATER
    // ========================================================================
    //
    // A library catalog:
    //   - 100 people can BROWSE the catalog at the same time ✅
    //   - But when the librarian UPDATES it, nobody can read! ✍️
    //
    // Mutex = only ONE person at the desk (even just to look)
    // RwLock = many people can browse, but updating closes the desk
    //
    println!("  ┌─────────────────────────────────────────────────────────┐");
    println!("  │  📚 SCENARIO 5: Library Catalog — Browse & Update      │");
    println!("  │  Problem: Readers blocking each other with Mutex       │");
    println!("  │  Solution: RwLock = many browsers, one updater         │");
    println!("  └─────────────────────────────────────────────────────────┘\n");
    let catalog = Arc::new(RwLock::new(vec![
        "📖 The Rust Programming Language",
        "📖 Programming Rust",
        "📖 Rust in Action",
        "📖 Zero to Production in Rust",
    ]));

    thread::scope(|s| {
        // 6 READERS — all browse simultaneously!
        for reader_id in 0..6 {
            let catalog_data = Arc::clone(&catalog);
            s.spawn(move || {
                let books = catalog_data.read().unwrap();
                let favorites = books[reader_id % books.len()];
                println!("The favourites is {} and the reader_id -{}", favorites, reader_id);
                thread::sleep(Duration::from_millis(50)); // Reading time
                // All 6 readers are reading AT THE SAME TIME! ✅
            });
        }
        // Wait a bit so readers start first
        thread::sleep(Duration::from_millis(20));
        let catalog_data = Arc::clone(&catalog);
        s.spawn(move || {
            println!("  📝 Librarian: Waiting for readers to finish...");
            let mut catalog_data_clone = catalog_data.write().unwrap();
            catalog_data_clone.push("📖 Rust Atomics and Locks (NEW!)");
            println!("  📝 Librarian: Added new book! Total: {}", catalog_data_clone.len());

        });
    });

    println!("  📚 Catalog now: {:?}", catalog.read().unwrap());
    println!("  💡 6 readers browsed simultaneously! Librarian waited politely.\n\n");

    Ok(())

}

pub fn multiple_read_write_resembling_hotel_booking() -> Result<(), Box<dyn std::error::Error>>{
    // STEP 1: Create the notebook with 5 rooms
    // true = FREE, false = BOOKED
    let notebook = Arc::new(RwLock::new(vec![true, true, true, true, true]));
    //             ^^^       ^^^^^^       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //             Share it   Lock it      5 rooms, all FREE

    println!("📒 Hotel notebook created: 5 rooms, all FREE\n");
    // STEP 2: Alice wants to READ the notebook
    let notebook_for_thread1 = Arc::clone(&notebook);  // Alice gets a pointer
    let thread1 = thread::spawn(move || {
        let rooms = notebook_for_thread1.read().unwrap();
        //                             ^^^^
        //                             READ lock — just looking!
        let free_count = rooms.iter().filter(|&&room| room==true).count();
        //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //               Go through each room, count the TRUE ones
        println!("👓 Alice: I see {} free rooms!", free_count);
        println!("👓 Alice: I'm still looking at the notebook...");
        thread::sleep(Duration::from_millis(500));  // Alice looks for 500ms
        println!("👓 Alice: Done looking!\n");

    });

    let notebook_for_thread2 = Arc::clone(&notebook);

    //thread2 also wants to read
    let thread2 = thread::spawn(move || {
        let rooms = notebook_for_thread2.read().unwrap();  // 👓 "Let me LOOK too"
        //                          ^^^^
        //                          ALSO a read lock — Bob goes in WITH Alice!
        let free_count = rooms.iter().filter(|&&room| room == true).count();
        println!("👓 Bob:   I see {} free rooms!", free_count);
        println!("👓 Bob:   I'm also looking at the same notebook...");
        thread::sleep(Duration::from_millis(500));
        println!("👓 Bob:   Done looking!\n");
    });

    //thread3 wants to write
    let notebook_for_thread3 = Arc::clone(&notebook);
    let thread3 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); // Carol arrives slightly later
        let mut rooms = notebook_for_thread3.write().unwrap();  // ✏️ "Let me WRITE"
        //                                 ^^^^^
        //                                 WRITE lock — needs EXCLUSIVE access!
        //                                 Waits until Alice AND Bob are done!
        rooms[2] = false;  // Room 3 (index 2) is now BOOKED!
        //    ^^^   ^^^^^
        //    Room 3  BOOKED (false)

 

    });

    //join the threads
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    let rooms = notebook.read().unwrap();

    for (i, &j) in rooms.iter().enumerate(){

        if j {
            println!("The room not free is {}", i+1)

        }
        else{

            println!("The room free is {}", i+1)
        }
        
    }





    Ok(())
}

