#![feature(lookup_host)]

fn main() {
    let filename = "domains.txt";

    for domain in &load_names(filename).unwrap_or_else( |e| { panic!("e"); } ) {
        print_ips(domain, &lookup_ip(domain).unwrap() );
        println!(" ");
    }

    println!("Press any key to continue...");
    use std::io::Read;
    let mut end_char : [u8; 1] = [0; 1];
    std::io::stdin().read_exact(&mut end_char);
}

fn load_names(filename :&str) -> Result<Vec<String>, &'static str> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(filename)
                .unwrap_or_else(|e| { 
                        panic!{"Unable to open file: \"{}\". Error: {}", filename, e};
                    } );

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|e| {
            panic!{"Unable to read contents of file: \"{}\". Error {}", filename, e}; 
        });

    let v = contents.lines().map(|s| { s.to_string() } ).collect();
    Ok(v)
}

fn lookup_ip(domain: &str) -> Result<Vec<String>, &'static str> {
    use std::net::lookup_host;

    match lookup_host(domain) {
        Ok(v) => Ok( v.map(|h| { h.ip().to_string() }).collect() ),
        Err(e) => Err("Error")
    }
}

fn print_ips(domain: &String, ips: &Vec<String>) {
    println!("{}", domain);

    for v in ips {
        println!("{}", v);
    }
}