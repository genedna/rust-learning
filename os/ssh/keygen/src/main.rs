extern crate openssl;
extern crate pem;
extern crate base64;
extern crate openssh_keys;
extern crate crypto;

use openssl::rsa::Rsa;
use openssl::pkey::Private;
use openssh_keys::PublicKey;
use pem::{Pem, encode};
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
  // Generate a new 4096-bit key.
  let rsa = Rsa::generate(4096).unwrap();

  let e = rsa.e();
  let n = rsa.n();
  
  println!("{}", private_key_to_pem_string(&rsa));
  println!("{}", public_key_to_string(e.to_vec(), n.to_vec(), &String::from("eli@patch.sh")));
  println!("{}", fringerprint_md5_string(e.to_vec(), n.to_vec()));
  
}

fn private_key_to_pem_string(rsa: &Rsa<Private>) -> String {
  let private_key = rsa.private_key_to_der().unwrap();
  let private_pem = Pem {
    tag: String::from("RSA PRIVATE KEY"),
    contents: private_key,
  };

  encode(&private_pem)
}

fn public_key_to_string(e: Vec<u8>, n: Vec<u8>, comment: &String) -> String {
  let mut key = PublicKey::from_rsa(e, n);
  key.set_comment(comment);
  key.to_string()
}

fn fringerprint_md5_string(e: Vec<u8>, n: Vec<u8>) -> String {
  let key = PublicKey::from_rsa(e, n);
  let mut sh = Md5::new();
  sh.input(&key.data());
  let mut output = [0; 16];
  sh.result(&mut output);
  
  fingerprint_legacy_md5_format(&output)
}

fn fingerprint_legacy_md5_format(fringerprint: &[u8; 16]) -> String {
  let md5: Vec<String> = fringerprint.iter()
    .map(|n| format!("{:02x}", n)).collect();
  
  md5.join(":")
}