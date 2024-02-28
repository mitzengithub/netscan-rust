use crate::netdiscovery::{ProviderModel, NetworkTask};

pub struct TraceRouter { 
}

impl NetworkTask for TraceRouter {
    fn execute(&self, s: &ProviderModel) {
        println!("Executing traceroute provider: {}", s.host);
    }
}