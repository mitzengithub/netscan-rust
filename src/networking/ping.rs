use crate::netdiscovery::{ProviderModel, NetworkTask};
use std::net::{IpAddr, Ipv4Addr};
use std::thread;
use tracert::ping::Pinger;

pub struct PingProvider { 

}

impl NetworkTask for PingProvider {
    fn execute(&self, s: &ProviderModel) {
        
        
    }
}

