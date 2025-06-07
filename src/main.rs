use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;

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

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("Usage: {} -j <threads> <ipaddr> or -h for help", program);
            std::process::exit(0);
        }
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}

// ip_sniffer.exe -h
// ip_sniffer.exe -j 100 192.168.1.1
// ip_sniffer 192.168.1.1
