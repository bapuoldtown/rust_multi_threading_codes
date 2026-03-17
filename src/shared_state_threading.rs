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