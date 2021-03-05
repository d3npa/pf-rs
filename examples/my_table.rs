use std::fs;
use std::error::Error;
use std::net::IpAddr;
use pf_rs::*;

static GREEN: &str = "\x1b[92m";
static RESET: &str = "\x1b[0m";


fn main() -> Result<(), Box<dyn Error>> {
    let fd = fs::OpenOptions::new()
        .write(true)
        .open("/dev/pf")?;

    let mut addrs = vec![
        PfrAddr::from_addr(IpAddr::V4("127.0.0.1".parse()?), 32),
        PfrAddr::from_addr(IpAddr::V4("127.0.0.2".parse()?), 32),
        PfrAddr::from_addr(IpAddr::V4("127.0.0.3".parse()?), 32),
        PfrAddr::from_addr(IpAddr::V6("::1".parse()?), 128),
    ];

    let mut table = PfTable::new("my_table");

    // Add a list of addresses to table
    table.add_addrs(&fd, addrs.clone())?;

    // Delete a list of addresses from table
    let last = addrs.pop().unwrap()
    table.del_addrs(&fd, vec![last])?;

    // Print contents of table
    println!("{}", table);

    // Clear all addresses from table
    table.clr_addrs(&fd)?;

    Ok(())
}