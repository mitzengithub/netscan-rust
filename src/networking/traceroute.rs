use crate::netdiscovery::{ProviderModel, NetworkTask};

pub struct TraceRouter { 
}

impl NetworkTask for TraceRouter {
    fn execute(&self, s: &ProviderModel) {
        println!("executing traceroute provider: {}", s.host);
    }
}