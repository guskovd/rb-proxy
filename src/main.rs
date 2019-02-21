extern crate clap;
extern crate rb_proxy;

use clap::{Arg, App, AppSettings};

fn main() {
    let matches = App::new("rb-proxy")
        .version("0.0.1")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("address")
             .long("address")
             .default_value("0.0.0.0")
             .help("The address to bind to")
        )
        .arg(Arg::with_name("port")
             .long("port")
             .default_value("8080")
             .help("The port to listen on")
        )
        .arg(Arg::with_name("proxy_port_range")
             .long("proxyPortRange")
             .default_value("8081..8581")
             .help("The range of ports to use for proxies")
        )
        .arg(Arg::with_name("ttl")
             .long("ttl")
             .default_value("0")
             .help("Time in seconds until an unused proxy")
        )
        .get_matches();

    let port = matches.value_of("port");
    rb_proxy::app::app(port.unwrap().parse().unwrap());
}
