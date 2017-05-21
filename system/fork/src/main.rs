extern crate nix;
extern crate time;

use nix::unistd::{fork, getpid, getppid};
use nix::unistd::ForkResult::{Parent, Child};

use time::{ precise_time_s };

fn main() {
	run(10000000);

	println!("");

	run(1000000000);
}

fn run(len: usize) {
	println!("Starting allocate memory");
	let t = start_timer();

    let a = vec![12; len];
    
    stop_timer(t);
    println!("Allocating memory finished\n");


    println!("Starting fork child process");
	let t = start_timer();

    let pid = fork();
    match pid {
        Ok(Child) => {
            println!("in child process with pid: {} and parent pid:{}", getpid(), getppid());
            std::process::exit(0);
        }
        Ok(Parent{child}) => {
        	stop_timer(t);
        	println!("in parent process: Forking finished");
            println!("in parent process with pid: {} and child pid:{}", getpid(), child);
        }
        // panic, fork should never fail unless there is a serious problem with the OS
        Err(_) => panic!("Error: Fork Failed")
    }	
}

fn start_timer() -> f64 {
    precise_time_s()
}

fn stop_timer(t: f64) {
    println!("time elapse {} second", precise_time_s() - t);
}