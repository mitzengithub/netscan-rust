use crate::netdiscovery::{ProviderModel, NetworkTask};
use std::net::{IpAddr, Ipv4Addr};
use std::thread;
use tracert::ping::Pinger;

pub struct TcpPingProvider { 

}

impl NetworkTask for TcpPingProvider {

    fn execute(&self, s: &ProviderModel) {

        println!("TcpPringProvider");
        
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

