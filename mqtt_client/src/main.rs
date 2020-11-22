mod mqtt_client {
  use clap::{App, Arg};
    pub struct Config {
        host: String,
        port: String,
        net_id: String,
        action: String,
        message: String,
        verbose: bool,
    }
    pub fn parse_arg() -> Config {
        let matches = App::new("MQTT Client")
            .version("0.1.0")
            .author("Walker Thornley")
            .about("Allows sending a single message to an MQTT broker")
            .arg(
                Arg::with_name("VERBOSE")
                    .short("v")
                    .takes_value(false)
                    .help("A cool file"),
            )
            .arg(
                Arg::with_name("HOST")
                    .short("h")
                    .long("host")
                    .takes_value(true)
                    .help("A cool file"),
            )
            .arg(
                Arg::with_name("PORT")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .help("Five less than your favorite number"),
            )
            .arg(Arg::with_name("NET_ID").required(true).takes_value(true))
            .arg(Arg::with_name("ACTION").required(true).takes_value(true))
            .arg(Arg::with_name("MESSAGE").required(true).takes_value(true))
            .get_matches();
        let host = String::from(matches.value_of("HOST").unwrap_or("localhost"));
        let port = String::from(matches.value_of("PORT").unwrap_or("8883"));
        let net_id = String::from(matches.value_of("NET_ID").unwrap());
        let action = String::from(matches.value_of("ACTION").unwrap());
        let message = String::from(matches.value_of("MESSAGE").unwrap());
        let verbose = matches.is_present("VERBOSE");

        let cnfg = Config {
            host: host,
            port: port,
            net_id: net_id,
            action: action,
            message: message,
            verbose: verbose,
        };

        if cnfg.verbose {
            println!("host: {}", cnfg.host);
            println!("port: {}", cnfg.port);
            println!("net_id: {}", cnfg.net_id);
            println!("action: {}", cnfg.action);
            println!("messag: {}", cnfg.message);
        }

        cnfg
    }
    pub fn connect() {}
    pub fn subscribe() {}
    pub fn receive() {}
    pub fn send_request() {}
    pub fn delivered() {}
    pub fn close() {}
}

fn main() {
    let cnfg = mqtt_client::parse_arg();

}
