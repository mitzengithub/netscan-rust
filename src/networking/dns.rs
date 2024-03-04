use crate::netdiscovery::{ProviderModel, NetworkTask};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::net::IpAddr;

pub struct DnsProvider {
}

impl DnsProvider { 

    fn extract_ip_address(&self, ip: IpAddr) -> String {
        match ip {
            IpAddr::V4(ipv4) => format!("{:?}", ipv4),
            IpAddr::V6(ipv6) => format!("{:?}", ipv6),
        }
    }

    pub fn resolve_host(&self, host: &str) -> IpAddr {

         // Construct a new Resolver with default configuration options
         let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

         // On Unix/Posix systems, this will read the /etc/resolv.conf
         // let mut resolver = Resolver::from_system_conf().unwrap();
         // Lookup the IP addresses associated with a name.
         let response = resolver.lookup_ip(host).unwrap();
 
         let address: IpAddr = response.iter().next().expect("no addresses returned!");

         return address;
    }
}

impl NetworkTask for DnsProvider {

    fn execute(&self, s: &ProviderModel) {
        println!("DNS Provider");
        let address = self.resolve_host(&s.host);
        println!("Ip resolved:{}", address.to_string());
    }    
}