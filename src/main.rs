// use core::num;
// use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments provided");
        } else if args.len() > 4 {
            return Err("Too many arguments provided");
        }
        let f = args[1].clone();
        if let Ok(IpAddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr: IpAddr::from_str(&f).unwrap(),
                threads: 1,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "usage: -j to select how many threads you want -h or -help to show this message"
                );
                return Err("Help requested");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments provided");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(ip) => ip,
                    Err(_) => return Err("Invalid IP address format"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(t) => t,
                    Err(_) => return Err("Invalid number of threads"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("Invalid flag or argument");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_thread: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap(); // Ensure the dot is printed immediately
                // Port is open, send it through the channel
                tx.send(port).unwrap();
            }
            Err(_) => {
                // Port is closed or filtered, do nothing
            }
        }
        if (MAX - port) <= num_thread {
            break; // Exit the loop if we have scanned all ports
        }
        port += num_thread; // Increment port by the number of threads
    }
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {   
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            std::process::exit(0);
        }
        // eprintln!("Error: {}", err);
        // std::process::exit(1);
    });
    let num_threads = arguments.threads;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx); // Close the sending end to prevent deadlock
    for port in rx {
        out.push(port);
    }
    for v in out {
        println!("Port {} is open", v);
    }
    println!("finished scanning")
}
