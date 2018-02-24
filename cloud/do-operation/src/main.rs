#[macro_use]

extern crate clap;
extern crate digitalocean;

use std::str::FromStr;
use std::usize;

use clap::{App, Arg, SubCommand};
use digitalocean::DigitalOcean;
use digitalocean::api::Droplet;
use digitalocean::request::Executable;

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
            .arg(Arg::with_name("id")
              .help("Droplet ID")
              .short("i")
              .long("id")
              .takes_value(true)
              .required(true))
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
        ("create", Some(_create_droplet_matches)) =>{
          crete_droplet(&api_key);
        },
        ("delete", Some(delete_droplet_matches)) =>{
          delete_droplet(&api_key, usize::from_str_radix(delete_droplet_matches.value_of("id").unwrap(), 10).unwrap());
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

fn list_droplet(api_key: &str) -> () {
  let client = DigitalOcean::new(api_key).unwrap();
  let droplets = Droplet::list().execute(&client);

  for droplet in droplets.iter(){
    println!("{:#?}", droplet);
  }
}

fn crete_droplet(api_key: &str) -> () {
  let mut keys : Vec<String> = Vec::new();
  keys.push(String::from_str("34:ef:6f:da:1f:01:64:13:35:e4:86:b8:4f:97:16:7d").unwrap());

  let mut tags : Vec<String> = Vec::new();
  tags.push(String::from_str("develop").unwrap());

  let client = DigitalOcean::new(api_key).unwrap();
  let droplet = Droplet::create("do-operation", "nyc3", "s-1vcpu-1gb", "centos-7-x64")
                  .ssh_keys(keys)
                  .backups(true)
                  .monitoring(true)
                  .ipv6(true)
                  .private_networking(true)
                  .tags(tags)
                  .execute(&client)
                  .unwrap();

  println!("Droplet id {:?}", droplet.id());
}

fn delete_droplet(api_key: &str, id: usize) -> () {
  let client = DigitalOcean::new(api_key).unwrap();
  let request = Droplet::delete(id).execute(&client);

  println!("{:#?}", request);
}

fn list_lb(api_key: &str) -> () {
  println!("{}", api_key);
}

fn create_lb(api_key: &str) -> () {
  println!("{}", api_key);
}

fn delete_lb(api_key: &str) -> () {
  println!("{}", api_key);
}