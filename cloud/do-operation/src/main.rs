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
            .arg(Arg::with_name("cpu")
              .help("Droplet CPU size")
              .short("c")
              .long("cpu")
              .takes_value(true)
              .required(true))
        )
        .subcommand(
          SubCommand::with_name("delete")
            .about("Delete a droplet in Digitalocean")  
        ))
    .subcommand(
      SubCommand::with_name("loadbalancer")
        .about("Digitalocean loadbalancer operations"))
    .get_matches();

  let mut api_key = String::new();

  if let Some(key) = matches.value_of("key") {
    api_key = key.to_string();
  }

  match matches.subcommand(){
    ("droplet", Some(droplet_matches)) =>{
      match droplet_matches.subcommand() {
        ("list", Some(_list_droplet_matches)) =>{
          list_droplet(&api_key);
        },
        ("create", Some(create_droplet_matches)) =>{
          crete_droplet(&api_key);
          println!("Pushing to {}", create_droplet_matches.value_of("cpu").unwrap());
        },
        ("delete", Some(_delete_droplet_matches)) =>{
          delete_droplet(&api_key);
        },
        _ => unreachable!(),
      }
    },
    ("loadbalancer", Some(lb_matches)) =>{
      match lb_matches.subcommand() {
        ("list", Some(_list_lb_matches)) =>{
          list_lb(&api_key);
        },
        ("create", Some(_create_lb_matches)) =>{
          create_lb(&api_key);
        },
        ("delete", Some(_delete_lbt_matches)) =>{
          delete_lb(&api_key);
        },
        _ => unreachable!(),
      }
    },
    ("", None)   => println!("No subcommand was used"),
    _ => unreachable!(),
  }
}

fn list_droplet(key: &str) -> () {
  println!("{}", key);
}

fn crete_droplet(key: &str) -> () {
  println!("{}", key);
}

fn delete_droplet(key: &str) -> () {
  println!("{}", key);
}

fn list_lb(key: &str) -> () {
  println!("{}", key);
}

fn create_lb(key: &str) -> () {
  println!("{}", key);
}

fn delete_lb(key: &str) -> () {
  println!("{}", key);
}