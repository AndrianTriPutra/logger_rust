use std::{
    process,
    env,
};

mod log;
mod inspect;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("argument not found");
        process::exit(1);
    }
    let access  = args[1..2].join(" ");
    let mode = args[2..3].join(" ");
    log::log::logger("info", "main",  &format!("access: [{}]", access));
    log::log::logger("info", "main",  &format!("mode  : {}", mode));
    println!("============================================================================");

    log::log::load(&access);

    log::log::logger("trace", "main", "this is trace");
    log::log::logger("debug", "main", "this is debug");
    log::log::logger("info", "main", "this is info");
    log::log::logger("warning", "main", "this is warning");
    log::log::logger("error", "main", "this is error");
    
    if mode=="0"{
        inspect::inspect::diagnostic_info(&format!("[main] mode: {}", mode));
        
        match inspect::inspect::diagnostic_fatal() {
            Ok(_) => log::log::logger("info", "main", "this is info from isnpect.diagnostic_fatal"),
            Err(e) => log::log::logger("fatal", "main", &format!("{}", e)),
        }
    }

    match inspect::inspect::diagnostic_panic() {
        Ok(_) => log::log::logger("info", "main", "this is info from isnpect.diagnostic_panic"),
        Err(e) => log::log::logger("panic", "main", &format!("{}", e)),
    }

}
