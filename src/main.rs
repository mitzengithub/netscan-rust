mod cmd;
mod netdiscovery;
mod networking;

fn main() {
    
    use clap::Parser;
    let cli = cmd::Cli::parse();

    println!("name: {:?}", cli.target.as_deref());
    
    // // You can check the value provided by positional arguments, or option arguments
    if let Some(hostname) = cli.target.as_deref() {
        println!("Value for name: {hostname}");
        
        let mut provider = netdiscovery::Provider::new(Vec::new());
        
        let model = netdiscovery::ProviderModel{
            host: hostname.to_string()
        };

        let ping = networking::ping::PingProvider{};
        let http = networking::http::HttpProvider{};
        let trt = networking::traceroute::TraceRouter{};

        provider.add_task(&ping);
        provider.add_task(&http);
        provider.add_task(&trt);

        provider.execute(&model);

        println!("{}", model.host);
    }    
}