mod cli;
pub mod dmg;
pub mod passwords;

use crate::dmg::Dmg;
use crate::passwords::read_password_list;
use clap::Parser;
use console::Emoji;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::env;
use std::sync::{Arc, RwLock};

static PADLOCK: Emoji<'_, '_> = Emoji("🔒", "");

pub fn run() {
    let args = cli::Args::parse();
    let passwords = read_password_list(&args.password_list_path).unwrap();
    println!("{PADLOCK} Attempting passwords...");
    let found_password = attempt_passwords_in_parallel(
        &passwords,
        &args.dmg_path,
        &args.thread_count,
    );
    match found_password {
        None => {
            println!("No password was found")
        }
        Some(i) => {
            println!("Password found: {i}")
        }
    }
}

fn attempt_passwords_in_parallel(
    passwords: &[String],
    dmg_path: &String,
    thread_count: &u8,
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

    let chunks = passwords
        .chunks(password_vec_size / usize::from(*thread_count))
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
                for p in chunk {
                    // check if another thread has found the password
                    let password_found_read = password_found.read().unwrap();
                    if !password_found_read.is_empty() {
                        return;
                    };
                    drop(password_found_read);

                    // check password and broadcast to other threads if found
                    let success = dmg.attempt_password(&p);
                    if success {
                        let mut pw_found_guard =
                            password_found.write().unwrap();
                        *pw_found_guard = p;
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
