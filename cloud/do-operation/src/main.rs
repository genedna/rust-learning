#[macro_use]

extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
      Arg::with_name("key")
      .help("Digitalocean API key")
      .short("k")
      .long("key")
      .takes_value(true)
      .required(true))
    .args_from_usage("-c,   --config=[FILE] 'Sets a custom config file'
                      -v... 'Sets the level of verbosity'
                      -d... 'Turn debugging information on'")
    .subcommand(
      SubCommand::with_name("droplet")
        .about("Digitalocean droplet operations")
        .subcommand(
          SubCommand::with_name("list")
            .about("List all digitalocean droplets")
        )
        .subcommand(
          SubCommand::with_name("create")
            .about("Create a droplet in Digitalocean")
        )
        .subcommand(
          SubCommand::with_name("delete")
            .about("Delete a droplet in Digitalocean")  
        ))
    .subcommand(
      SubCommand::with_name("loadbalancer")
        .about("Digitalocean loadbalancer operations"))
    .get_matches();

  let mut _api_key = String::new();

  if let Some(key) = matches.value_of("key") {
    _api_key = key.to_string();
  }

  if let Some(ref _droplet) = matches.subcommand_matches("droplet") {
    println!("{:?}", _api_key);
  }
}