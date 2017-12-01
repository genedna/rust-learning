package main

import (
	"bytes"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"os"
	"path"
	"path/filepath"

	"golang.org/x/crypto/ssh"
)

const (
	//KeyPath is folder include id_rsa and id_rsa.pub files
	KeyPath = "ssh"
	//PublicFileName is public key file name
	PublicFileName = "id_rsa.pub"
	//PrivateFileName is private key file name
	PrivateFileName = "id_rsa"
	//KeyBits4096 is key bits
	KeyBits4096 = 4096
)

func main() {
	dir, _ := filepath.Abs(filepath.Dir(os.Args[0]))
	private := path.Join(dir, KeyPath, PrivateFileName)
	_, _, fingerprint, _ := openSSHKeyFiles("", private)
	fmt.Println(fingerprint)
}

//isDirExist checks if a path is an existed dir
func isDirExist(path string) bool {
	fi, err := os.Stat(path)

	if err != nil {
		return os.IsExist(err)
	}

	return fi.IsDir()
}

//isFileExist checks if a file url is an exist file
func isFileExist(filename string) bool {
	_, err := os.Stat(filename)
	return err == nil || os.IsExist(err)
}

//generateSSHKeyFiles create SSH key in @src , default file name is id_rsa and id_rsa.pub.
//Will remove the SSH key files if there are in @src.
//And in generateSSHKeyFiles will return fingerprint.
func generateSSHKeyFiles(basePath string) (string, string, string, error) {
	sshPath := path.Join(basePath, KeyPath)
	publicFile := path.Join(sshPath, PublicFileName)
	privateFile := path.Join(sshPath, PrivateFileName)

	privateKey, err := rsa.GenerateKey(rand.Reader, KeyBits4096)
	if err != nil {
		return "", "", "", err
	}

	//generate private key
	var private bytes.Buffer
	privateKeyPEM := &pem.Block{Type: "RSA PRIVATE KEY", Bytes: x509.MarshalPKCS1PrivateKey(privateKey)}
	if err := pem.Encode(&private, privateKeyPEM); err != nil {
		return "", "", "", err
	}

	//generate public key
	pub, err := ssh.NewPublicKey(&privateKey.PublicKey)
	if err != nil {
		fmt.Println(err.Error())
	}
	public := ssh.MarshalAuthorizedKey(pub)
	fingerprint := ssh.FingerprintLegacyMD5(pub)

	//Remove exist public and private file
	if isFileExist(privateFile) == true {
		if err := os.Remove(privateFile); err != nil {
			return "", "", "", err
		}
	}

	if isFileExist(publicFile) == true {
		if err := os.Remove(publicFile); err != nil {
			return "", "", "", err
		}
	}

	//Save public key and private key file.
	if isDirExist(sshPath) == false {
		if err := os.MkdirAll(sshPath, os.ModePerm); err != nil {
			return "", "", "", err
		}
	}

	if err := ioutil.WriteFile(privateFile, private.Bytes(), 0400); err != nil {
		return "", "", "", err
	}
	if err := ioutil.WriteFile(publicFile, public, 0600); err != nil {
		return "", "", "", err
	}

	return publicFile, privateFile, fingerprint, nil
}

//openSSHKeyFiles open ssh public and private key files to valid.
//Then return the fingerprint.
func openSSHKeyFiles(publicFile, privateFile string) (string, string, string, error) {
	if isFileExist(privateFile) == false {
		return "", "", "", fmt.Errorf("Private key file not exist")
	}

	var privateByte, publicData []byte
	var err error
	var publicKey ssh.PublicKey
	var fingerprint string

	//Read private key file
	privateByte, err = ioutil.ReadFile(privateFile)
	if err != nil {
		return "", "", "", fmt.Errorf("Read privateFile key file error, %s", err.Error())
	}

	//Decode file byte[]
	blockPrivate, _ := pem.Decode(privateByte)
	rsaPrivate, err := x509.ParsePKCS1PrivateKey(blockPrivate.Bytes)

	if rsaPrivate.Validate() != nil {
		return "", "", "", fmt.Errorf("Valid privateFile key error")
	}

	//Get fingerprint from private key
	pub, err := ssh.NewPublicKey(&rsaPrivate.PublicKey)
	if err != nil {
		fmt.Println(err.Error())
	}
	f := ssh.FingerprintLegacyMD5(pub)

	if publicFile != "" {
		//Read public key file, and return fingerprint.
		publicData, err = ioutil.ReadFile(publicFile)
		if err != nil {
			return "", "", "", fmt.Errorf("Read public file key file error, %s", err.Error())
		}

		publicKey, _, _, _, err = ssh.ParseAuthorizedKey(publicData)
		if err != nil {
			return "", "", "", fmt.Errorf("Parse ssh public file key error")
		}

		fingerprint = ssh.FingerprintLegacyMD5(publicKey)
	} else {
		//No public key file provide, will create one form private key.
		publicSSHKey := ssh.MarshalAuthorizedKey(pub)
		publicFile := path.Join(path.Dir(privateFile), PublicFileName)

		if err := ioutil.WriteFile(publicFile, publicSSHKey, 0600); err != nil {
			return "", "", "", err
		}

		fingerprint = f
	}

	//If different fingerprint form public key and private key, will return error.
	if f != fingerprint {
		return "", "", "", fmt.Errorf("public and Ppivate key is not pair")
	}

	return publicFile, privateFile, fingerprint, nil
}
