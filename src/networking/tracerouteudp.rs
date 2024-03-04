use crate::netdiscovery::{ProviderModel, NetworkTask};
use std::net::{IpAddr, Ipv4Addr};
use std::thread;
use tracert::ping::Pinger;

pub struct TracerouteUdpProvider { 

}

impl NetworkTask for TracerouteUdpProvider {

    fn execute(&self, s: &ProviderModel) {

        println!("TracerouteUdpProvider");        
        // IPv6 TCP ping to cloudflare's one.one.one.one (2606:4700:4700::1111)
        //let dst_ip: IpAddr = IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111));
        let port: u16 = 80;
        let mut pinger: Pinger = Pinger::new(s.ip).unwrap();
        pinger.set_protocol(tracert::protocol::Protocol::Tcp);
        pinger.set_dst_port(port);
        let rx = pinger.get_progress_receiver();
        // Run ping
        let handle = thread::spawn(move || pinger.ping());
        // Print progress
        println!("Progress:");
        while let Ok(msg) = rx.lock().unwrap().recv() {
            println!(
                "{} {}:{} {:?} {:?}",
                msg.seq, msg.ip_addr, port, msg.hop, msg.rtt
            );
        }
        // Print final result
        println!("Result:");
        match handle.join().unwrap() {
            Ok(r) => {
                println!("Status: {:?}", r.status);
                for result in r.results {
                    println!("{:?}", result);
                }
                println!("Probe Time: {:?}", r.probe_time);
            }
            Err(e) => {
                print!("{}", e);
            }
        }
    }
}

