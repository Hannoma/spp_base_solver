pub mod solver;
pub mod config;

use std::thread;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc;
use std::time::Instant;
use config::Config;
use crate::solver::Solver;

pub fn main<R: 'static + Clone + std::marker::Send, T: 'static + std::marker::Send, S: Solver<R, T>>(config: Config, _solver: S) {
    // Start timer for run time analysis or stopping in time in battle mode
    let now = Instant::now();

    if config.battle_mode {
        let input = S::parse_input();

        // Save channels of threads to receive their messages later
        let mut channels = Vec::with_capacity(config.num_threads as usize);

        // Spawn all threads
        for _ in 0..config.num_threads {
            // Create new channel and save it
            let (sender, receiver) = mpsc::channel();
            channels.push(receiver);
            // Clone input
            let cloned_input = input.clone();
            // Spawn new thread
            thread::spawn(move || {
                sender.send(S::solve(&cloned_input)).ok();
            });
        }

        let mut best_solution = None;
        let mut best_weight: u64 = 0;

        // Wait for solutions
        while now.elapsed().as_secs() < (config.run_time - 2) as u64 {
            for receiver in channels.iter() {
                if let Ok(outcome) = receiver.try_recv() {
                    if let Some(_) = best_solution {
                        if config.maximize_weight && outcome.weight > best_weight ||
                            !config.maximize_weight && outcome.weight < best_weight {
                            best_solution = Some(outcome.solution);
                            best_weight = outcome.weight;
                        }
                    } else {
                        best_solution = Some(outcome.solution);
                        best_weight = outcome.weight;
                    }
                }
            }
        }

        // Format solution
        let solution = S::format_solution(&best_solution.expect("Not found any solutions in time ;("));
        // Print and write file
        print_and_save_string(solution);
    } else {
        // Here can stand verbose code
        let input = S::parse_input();
        let outcome= S::solve(&input);

        println!("Successfully solved input");
        let solution = S::format_solution(&outcome.solution);
        // Print and write file
        print_and_save_string(solution);
        println!();
        println!("Weight: {}", outcome.weight);
        println!("Elapsed: {} milliseconds", now.elapsed().as_millis());
    }
}

fn print_and_save_string(out: String) {
    println!("{}", out);
    let file = File::create("output").expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    writer.write_all(out.as_bytes()).expect("Unable to write data");
}