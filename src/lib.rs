mod cli;
pub mod dmg;
pub mod passwords;

use crate::dmg::Dmg;
use crate::passwords::read_password_list;
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::env;
use std::sync::{Arc, RwLock};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let passwords =
        read_password_list(&args.password_list_path).map_err(|e| {
            eprintln!(
                "❌ Error reading password list from '{}': {}",
                args.password_list_path, e
            );
            eprintln!("💡 Please check:");
            eprintln!("   - File exists and is readable");
            eprintln!("   - File format is correct (.txt or .csv)");
            eprintln!("   - You have permission to read the file");
            e
        })?;

    if passwords.is_empty() {
        eprintln!("❌ Error: Password list is empty");
        eprintln!("💡 Please ensure your password file contains at least one password");
        return Err("Password list is empty".into());
    }

    let mut password_list = passwords;
    if args.randomize {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        password_list.shuffle(&mut thread_rng());
        println!("🔀 Randomized password order");
    }

    println!(
        "🔒 Attempting {} passwords with {} threads...",
        password_list.len(),
        args.thread_count
    );
    let found_password = attempt_passwords_in_parallel(
        &password_list,
        &args.dmg_path,
        &args.thread_count,
    );
    match found_password {
        None => {
            println!("No password was found");
        }
        Some(password) => {
            println!("Password found: {password}");
        }
    }

    Ok(())
}

fn attempt_passwords_in_parallel(
    passwords: &[String],
    dmg_path: &String,
    thread_count: &usize,
) -> Option<String> {
    let password_vec_size = passwords.len();
    let mp = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta}) ({per_sec})",
    )
    .unwrap()
    .progress_chars("##-");

    let shared_mp = Arc::new(&mp);
    let shared_dmg_path = Arc::new(dmg_path);
    let shared_password_found = Arc::new(RwLock::new(String::new()));

    let chunk_size = if *thread_count == 0 || password_vec_size == 0 {
        1
    } else {
        std::cmp::max(1, password_vec_size / *thread_count)
    };

    let chunks = passwords
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    env::set_var("RAYON_NUM_THREADS", format!("{thread_count}"));
    chunks.into_par_iter().for_each(|chunk| {
        let pb =
            Arc::clone(&shared_mp).add(ProgressBar::new(chunk.len() as u64));
        pb.set_style(sty.clone());
        let password_found = Arc::clone(&shared_password_found);

        rayon::scope(|s| {
            s.spawn(|_| {
                let dmg = Dmg::new(Arc::clone(&shared_dmg_path).as_ref());
                const CHECK_FREQUENCY: usize = 10; // Check shared state every 10 passwords

                for (passwords_checked, password) in
                    chunk.into_iter().enumerate()
                {
                    // Check if another thread has found the password (less frequently)
                    if passwords_checked % CHECK_FREQUENCY == 0 {
                        let password_found_read =
                            password_found.read().unwrap();
                        if !password_found_read.is_empty() {
                            return;
                        }
                        drop(password_found_read);
                    }

                    // check password and broadcast to other threads if found
                    let success = dmg.attempt_password(&password);
                    if success {
                        let mut pw_found_guard =
                            password_found.write().unwrap();
                        *pw_found_guard = password;
                        return;
                    };

                    pb.inc(1);
                }
            });
        });
    });

    let found_password = shared_password_found.read().unwrap();
    match found_password.is_empty() {
        true => None,
        false => Some(found_password.clone()),
    }
}
