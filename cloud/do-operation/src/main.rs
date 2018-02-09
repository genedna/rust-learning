#[macro_use]

extern crate clap;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'
                      -v... 'Sets the level of verbosity'
                      -d... 'Turn debugging information on'
                      -k... 'Digitalocean API key'")
    .subcommand(SubCommand::with_name("droplet")
      .about("Digitalocean droplet operations"))
    .subcommand(SubCommand::with_name("loadbalancer")
      .about("Digitalocean loadbalancer operations"))
    .get_matches();

  match matches.subcommand_name() {
    Some("droplet")      => println!("management digitalocean droplet"),
    Some("loadbalancer") => println!("management digitalocean loadbalancer"),
    None                 => println!("No subcommand was used"),
    _                    => unreachable!(),
  }
}
