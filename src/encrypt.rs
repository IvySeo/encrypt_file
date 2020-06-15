extern crate libsodium_sys as ffi;
extern crate std;
#[cfg(not(feature = "std"))]
use crate::write_file;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sodiumoxide::crypto::{
    box_::{gen_keypair},
    sealedbox,
};
extern crate libc;
use anyhow::{anyhow, Result};
use base64;
use sodiumoxide::crypto::{
    box_::{curve25519xsalsa20poly1305::PublicKey, curve25519xsalsa20poly1305::SecretKey}
    
};


pub fn encrypt_start(pk: String)  -> Result<String>  {
    // ------------------------------------------ GET JSON TEXT FILE CONTENTS ------------------------------------------------------------------------
    
            // specify the filename and its path to open and read   -- just for now, so just focus on encrypt and decrypt the file - later work on recieve the file I guess?
            let filename = "./example_json_txt_file_to_encrypt";
    
            // Open the file in read-only mode (ignoring errors).
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);

            let mut encrypted = "temp encrypted".to_string();
            let mut FILE_NAME_UUID: String  = "filename_uuid".to_string();

            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (index, line) in reader.lines().enumerate() {
                let message = line.unwrap(); // Ignore errors.
                //println!("read file: {:?}", message);
    
    // ------------------------------------------ PUBLIC KEY AND ENCRYPT IT ------------------------------------------------------------------------
    
                let pk_decoded = base64::decode(pk.clone())?;
                //println!("decoded pk: {:?}", pk_decoded);

                let public_key = PublicKey::from_slice(&pk_decoded)
                .ok_or_else(|| anyhow!("unable to create public key object"))?;
                //println!("pk from slice: {:?}", public_key);
    
                let encrypted_result = sealedbox::seal(message.as_bytes(), &public_key);
                //println!("encrypted_result: {:?}", encrypted_result);             

                encrypted = format!("{:?}", encrypted_result);

                let UUID_ = write_file::write_file(encrypted_result);  
                match UUID_ {
                    Ok(UUID) => {
                        println!("UUID: {:?}", UUID);
                        FILE_NAME_UUID = UUID;
                    }
                    Err(err) => println!("error to write encrypted file : {:?}", err)
                }

            }

            Ok(FILE_NAME_UUID)
        }
    

        pub fn generate_pk_sk() -> String {

            let (pk, sk) = gen_keypair();
            //println!("pk: {:?}\nsk: {:?}", pk, sk);
    
            let PublicKey(pk_bytes) = pk;
            let SecretKey(sk_bytes) = sk;
    
            let pk_encoded = base64::encode(pk_bytes);
            let sk_encoded = base64::encode(sk_bytes);
            //println!("encoded pk: {:?}\nencoded sk: {:?}", pk_encoded, sk_encoded);
    
            write_file::write_pk_file(pk_encoded.clone());
            write_file::write_sk_file(sk_encoded);
    
            let pk_encoded_return = pk_encoded.clone();
    
            //println!("pk sk generated");
    
            return pk_encoded_return;
    
        }
    
    