use crate::netdiscovery::{ProviderModel, NetworkTask};

pub struct HttpProvider {
}

impl NetworkTask for HttpProvider {
    fn execute(&self, s: &ProviderModel) {
        let h = HttpHandler{};
        h.send_get_request(s);
    }
}

pub struct HttpHandler {
}

impl HttpHandler {
    fn send_get_request(self, host: &ProviderModel) {
        let endpoint = format!("https://{}", host.host);
        let body = reqwest::blocking::get(endpoint);
        println!("Endpoint: body = {:?}", body);
    }     
}

