use crate::bridge::*;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn convert_pfaddr_from_c() {
    let mut c_addr = pfr_addr::init();
    c_addr.pfra_af = AF_INET;
    c_addr.pfra_u._pfra_ip4addr = u32::from_le_bytes([127, 0, 0, 1]);
    c_addr.pfra_net = 32;

    let pf_addr = PfAddr::try_from(c_addr).unwrap();
    assert_eq!(pf_addr.addr, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(pf_addr.ifname, String::from(""));
    assert_eq!(pf_addr.subnet, 32);
}

#[test]
fn convert_pfaddr_into_c() {
    let pf_addr = PfAddr {
        addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        ifname: String::from(""),
        subnet: 32,
    };

    let c_addr: pfr_addr = PfAddr::into(pf_addr);
    assert_eq!(c_addr.pfra_af, AF_INET);
    assert_eq!(
        unsafe { c_addr.pfra_u._pfra_ip4addr }, 
        u32::from_le_bytes([127, 0, 0, 1]),
    );
    assert_eq!(c_addr.pfra_net, 32);
}
