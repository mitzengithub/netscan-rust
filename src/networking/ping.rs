use crate::netdiscovery::{ProviderModel, NetworkTask};

pub struct PingProvider { 
}

impl NetworkTask for PingProvider {
    fn execute(&self, s: &ProviderModel) {
        println!("executing ping provider from trait: {}", s.host);
    }
} 