mod cmd;
mod netdiscovery;


fn main() {

    use clap::Parser;
    let cli = cmd::Cli::parse();

    println!("name: {:?}", cli.target.as_deref());
    
    // // You can check the value provided by positional arguments, or option arguments
    if let Some(hostname) = cli.target.as_deref() {
        println!("Value for name: {hostname}");
    }
    
    // // if let Some(config_path) = cli.config.as_deref() {
    // //     println!("Value for config: {}", config_path.display());
    // // }
    
    // // You can see how many times a particular flag or argument occurred
    // // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }
    
    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level cmd
    // match &cli.command {
    //     Some(cmd::Commands::Test { list }) => {
    //         if *list {
    //             println!("Printing testing lists...");
    //         } else {
    //             println!("Not printing testing lists...");
    //         }
    //     }
    //     None => {}
    // }    
}