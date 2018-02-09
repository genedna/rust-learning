#[macro_use]

extern crate clap;

use std::process;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .args_from_usage("-k,   --key=[API_KEY] 'Sets a Digitalocean API key'
                      -c... --config=[FILE] 'Sets a custom config file'
                      -v... 'Sets the level of verbosity'
                      -d... 'Turn debugging information on'")
    .subcommand(SubCommand::with_name("droplet")
      .about("Digitalocean droplet operations"))
    .subcommand(SubCommand::with_name("loadbalancer")
      .about("Digitalocean loadbalancer operations"))
    .get_matches();

  let mut _api_key = String::new();

  if let Some(key) = matches.value_of("key") {
    _api_key = key.to_string();
  } else {
    println!("Operate digitalocean API needs ");
    process::exit(-1);
  }

  match matches.subcommand_name() {
    Some("droplet")      => println!("management digitalocean droplet"),
    Some("loadbalancer") => println!("management digitalocean loadbalancer"),
    None                 => println!("No subcommand was used"),
    _                    => unreachable!(),
  }

  process::exit(0);
}
