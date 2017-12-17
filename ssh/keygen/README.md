## SSH2 Format Public & Private Key Generate

### Features

1. Generate SSH2 format public & private key
2. Save key data to files
3. Calucate SSH2 key fingerprint

### Support Key Types

- [X] ssh-rsa 1024/2048/4096
- [ ] ssh-dss
- [ ] ssh-ed25519
- [ ] ecdsa-ssh2-nistp256
- [ ] ecdsa-ssh2-nistp384
- [ ] ecdsa-ssh2-nistp521

### Linux Dependency

In Linux, the build need `openssl-devel` or `libssl-dev`.

```
# On Debian and Ubuntu
sudo apt-get install pkg-config libssl-dev
# On Arch Linux
sudo pacman -S openssl
# On Fedora
sudo dnf install openssl-devel
```

### Windows Dependency

In Windows, the build need install _**OpenSSL**_ with `vcpkg install openssl` command, then set environment variables `OPENSSL_DIR` and `OPENSSL_LIB_DIR`. 

```
OPENSSL_DIR = C:\Users\path\vcpkg\packages\openssl_x64-windows
OPENSSL_LIB_DIR = C:\Users\path\vcpkg\packages\openssl_x64-windows\lib
```

#### How to install vcpkg 

Intall [vcpkg](https://github.com/Microsoft/vcpkg) like this,

```
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
```

Then set environment variable `VCPKG_ROOT` to _vcpkg_ path.

### Contribute

* [openssh-keys](https://github.com/sdemos/openssh-keys)
  * [#9](https://github.com/sdemos/openssh-keys/pull/9)
