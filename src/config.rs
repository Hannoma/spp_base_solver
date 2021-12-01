use std::env;

pub struct Config {
    pub(crate) battle_mode: bool,
    pub(crate) num_threads: u8,
    pub(crate) restart_threads: bool,
    pub(crate) run_time: u16,
    pub(crate) maximize_weight: bool,
}

impl Config {
    pub fn new(num_threads: u8,
               restart_threads: bool,
               run_time: u16,
               maximize_weight: bool) -> Config {
        Config {
            battle_mode: !env::var("BATTLE_MODE").is_err(),
            num_threads,
            restart_threads,
            run_time,
            maximize_weight
        }
    }
}