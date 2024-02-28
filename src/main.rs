mod cmd;
mod netdiscovery;
mod networking;

fn main() {
    
    use clap::Parser;
    let cli = cmd::Cli::parse();

    let host = cli.target.as_deref();
    //let arg = String::from("www.google.com");
  
    if  let Some(hostname) = cli.target.as_deref() {
        println!("Value for name: {hostname}");

        let mut provider = netdiscovery::Provider::new(Vec::new());
                
        let ping = networking::ping::TcpPingProvider{};
        let http = networking::http::HttpProvider{};
        let trt = networking::traceroute::TraceRouter{};
        let dns = networking::dns::DnsProvider{};

        let address = dns.resolve_host(hostname);

        let model = netdiscovery::ProviderModel{
            host: hostname.trim().to_string(),
            ip : address
        };

        provider.add_task(&ping);
        provider.add_task(&http);
        provider.add_task(&trt);
        provider.add_task(&dns);
        provider.execute(&model);
        
        //println!("{}", model.host);
    }    
}

fn handle_tuple(tuple: (Option<String>, &str)) {
    match tuple {
        (Some(target), string2) if !target.is_empty() && !string2.is_empty() => {
            // Both strings have values, process them as needed
            println!("Both strings have values: {} and {}", target, string2);
        },
        (Some(target), "123") => {
            // Only the first string has a value
            println!("Only the first string has a value: {}", target);
        },
        (None, string2) => {
            // Only the second string has a value
            println!("Only the second string has a value: {}", string2);
        },
        _ => {
            // Both strings are empty
            println!("Both strings are empty");
        },
    }
}
