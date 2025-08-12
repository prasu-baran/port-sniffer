use std::{env, net::IpAddr, str::FromStr};

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
                    "Usage: -j to select how many threads you want\n\
                     -h or -help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    match Arguments::new(&args) {
        Ok(arguments) => {
            println!("Program: {}", program);
            println!("Flag: {}", arguments.flag);
            println!("IP Address: {}", arguments.ipaddr);
            println!("Threads: {}", arguments.threads);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
