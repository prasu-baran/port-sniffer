use std::{
    env,
    net::{IpAddr, SocketAddr, TcpStream},
    str::FromStr,
    thread,
    time::Duration,
};

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if (flag.contains("-h") || flag.contains("-help")) && args.len() == 2 {
                println!(
                    "Usage:\n\
                     <ip>                 Scan with default 4 threads\n\
                     -j <threads> <ip>    Scan with custom number of threads\n\
                     -h / -help           Show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                if args.len() != 4 {
                    return Err("Usage: -j <threads> <ip>");
                }
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    flag,
                    ipaddr,
                    threads,
                });
            }
        }
        Err("invalid syntax")
    }
}

// Port scanning worker
fn scan_ports(ip: IpAddr, start_port: u16, step: u16) {
    for port in (start_port..=65535).step_by(step as usize) {
        let addr = SocketAddr::new(ip, port);
        if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
            println!("Port {} is OPEN", port);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    match Arguments::new(&args) {
        Ok(arguments) => {
            println!("Program: {}", program);
            println!("Scanning IP: {}", arguments.ipaddr);
            println!("Threads: {}", arguments.threads);
            println!("Starting scan...\n");

            let mut handles = vec![];

            for i in 0..arguments.threads {
                let ip = arguments.ipaddr.clone();
                let threads = arguments.threads;
                handles.push(thread::spawn(move || {
                    scan_ports(ip, i + 1, threads);
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            println!("\nScan complete.");
        }
        Err(e) => {
            if e != "help" {
                eprintln!("Error: {}", e);
            }
        }
    }
}
