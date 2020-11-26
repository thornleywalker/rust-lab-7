mod mqtt_client {
    use clap::{App, Arg};
    use futures::executor::block_on;
    use paho_mqtt as mqtt;
    use std::{process, sync::mpsc, time::Duration};

    pub struct Config {
        pub host: String,
        pub port: String,
        pub net_id: String,
        pub action: String,
        pub message: String,
        pub verbose: bool,
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
        for letter in port.chars() {
            if !letter.is_numeric(){
                eprintln!("Invalid port number");
                std::process::exit(1);
            }
        }
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
            println!("message: {}", cnfg.message);
        }

        cnfg
    }

    pub struct MqttClient {
        net_id: String,
        action: String,
        message: String,
        client: mqtt::AsyncClient,
    }
    impl MqttClient {
        pub fn new(cnfg: Config) -> MqttClient {
            let host_uri = String::from("tcp://") + &cnfg.host + ":" + &cnfg.port;

            let create_opts = mqtt::CreateOptionsBuilder::new()
                .server_uri(host_uri)
                .client_id(&cnfg.net_id)
                .finalize();

            let client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
                println!("Error creating the client: {:?}", e);
                process::exit(1);
            });

            let cli = MqttClient {
                net_id: cnfg.net_id,
                action: cnfg.action,
                message: cnfg.message,
                client: client,
            };
            cli
        }

        pub fn connect(&mut self) {
            if let Err(err) = block_on(async {
                let conn_opts = mqtt::ConnectOptionsBuilder::new()
                    .keep_alive_interval(Duration::from_secs(20))
                    .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
                    .clean_session(true)
                    .finalize();
                self.client.connect(conn_opts).await?;

                Ok::<(), mqtt::Error>(())
            }) {
                eprintln!("{}", err)
            }
        }
        pub fn subscribe(&mut self) -> mpsc::Receiver<bool> {
            let (tx, rx) = mpsc::channel();
            self.client.set_message_callback(move |_cli, mess_opts| {
                match mess_opts {
                    Some(mess) => println!("{}", mess.payload_str()),
                    None => {}
                };

                tx.send(true).unwrap();
            });

            let mut topic = String::from("");
            topic = topic + &self.net_id + "/response";
            self.client.subscribe(topic, 1);
            rx
        }
        pub fn send_request(&mut self) {
            let pub_topic = String::from("") + &self.net_id + "/" + &self.action;
            let pub_message = String::from("") + &self.message;
            let msg = mqtt::Message::new(pub_topic, pub_message, mqtt::QOS_1);
            self.client.publish(msg);
        }
        pub fn close(&self) {
            self.client.disconnect(None);
        }
    }
}

fn main() {
    let cnfg = mqtt_client::parse_arg();

    let mut client = mqtt_client::MqttClient::new(cnfg);

    client.connect();
    let rx = client.subscribe();

    client.send_request();

    loop {
        match rx.recv() {
            Ok(val) => {
                if val {
                    break;
                }
            }
            Err(_err) => {}
        }
    }
    client.close();
}
