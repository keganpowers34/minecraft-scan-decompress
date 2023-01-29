use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

fn main() {

    let file = std::fs::read("ips").unwrap();
    let mut ips = Vec::new();
    for i in (0..file.len()).step_by(6) {
        let mut ip = String::from("");
        for j in 0..4 {
            let octet = u8::from_be(file[i + j]);
            ip.push_str(&octet.to_string());
            if j != 3 {
                ip.push_str(".");
            }
     
        }
        ip.push_str(":");
        let port = u16::from_be_bytes([file[i + 4], file[i + 5]]);
        ip.push_str(&port.to_string());
        ips.push(ip);
    }    
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());  
    let mut output_file = File::create("output.txt");
    let mut output_file = OpenOptions::new().write(true).open("./output.txt").unwrap();
    for ip in ips {
        output_file.write_all(ip.as_bytes()).unwrap();
        output_file.write_all(b"\n").unwrap();
    }

} 
