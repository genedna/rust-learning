### SSH With Rust

Recently, I am working on [ops](https://github.com/patchlab/ops) with [Rust](https://www.rust-lang.org), I usually *initiate & update & cleanup* the server within SSH connection without using Salt, Puppet or others. I generate SSH keypair files, connect the server with SSH connection, execute commands and *upload* or *download* command through the SSH connection.

#### Concept Resource

##### English

##### Chinese

- https://blog.cnbluebox.com/blog/2014/03/19/rsajia-mi
- http://www.ruanyifeng.com/blog/2013/06/rsa_algorithm_part_one.html
- http://www.ruanyifeng.com/blog/2013/07/rsa_algorithm_part_two.html

#### Libraries

- https://github.com/getreu/rustlang-play-rsa _Generate RSA key without with hardcode_
- https://github.com/sdemos/openssh-keys _Read & write public key library_
