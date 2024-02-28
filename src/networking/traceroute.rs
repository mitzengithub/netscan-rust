use crate::netdiscovery::{ProviderModel, NetworkTask};
use std::net::{IpAddr, Ipv4Addr};
use std::thread;
use tracert::trace::Tracer;

pub struct TraceRouter { 
}

impl NetworkTask for TraceRouter {

    fn execute(&self, s: &ProviderModel) {

        println!("Traceroute provider: {}", s.host);
        //IPv6 UDP traceroute to dns.google (2001:4860:4860::8888)
        //let dst_ip: IpAddr = IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888));
        let tracer: Tracer = Tracer::new(s.ip).unwrap();
        let rx = tracer.get_progress_receiver();
        // Run trace
        let handle = thread::spawn(move || tracer.trace());
        // Print progress
        println!("Progress:");
        while let Ok(msg) = rx.lock().unwrap().recv() {
            println!("{} {} {:?} {:?}", msg.seq, msg.ip_addr, msg.hop, msg.rtt);
        }
        // Print final result
        println!("Result:");
        match handle.join().unwrap() {
            Ok(r) => {
                println!("Status: {:?}", r.status);
                for node in r.nodes {
                    println!("{:?}", node);
                }
                println!("Trace Time: {:?}", r.probe_time);
            }
            Err(e) => {
                print!("{}", e);
            }
    }

    }
}