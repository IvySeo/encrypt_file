
// ------------------------------------------ WRITE A FILE ------------------------------------------------------------------------

extern crate libsodium_sys as ffi;
extern crate std;
#[cfg(not(feature = "std"))]
use std::fs::File;
use std::io::{BufWriter, Write};
extern crate libc;
use uuid::Uuid; 

pub fn write_file(encrypted:  Vec<u8>) -> std::io::Result<String>{
        
    let my_uuid = Uuid::new_v4();
    let mut file_created = File::create(format!("C:/Users/M7/Desktop/encrypt_file/encrypted_files/{}", my_uuid)).expect("create file failed");

    file_created.write_all(&encrypted)?;

    println!("encrypted txt file generated. uuid: {:?}", my_uuid);

    Ok(my_uuid.to_string())

}

pub fn write_pk_file(pk: String){
    
    //let my_uuid = Uuid::new_v4();
    let file_created = File::create("C:/Users/M7/Desktop/encrypt_file/public_key/pk").expect("create pk file failed");

    // buffer write
    let mut file_generated = BufWriter::new(file_created);

    //  write file
    file_generated.write_all(pk.as_bytes()).expect("write failed");

    //println!("pk txt file generated.");

}

pub fn write_sk_file(sk: String){

    let file_created = File::create("C:/Users/M7/Desktop/encrypt_file/secret_key/sk").expect("create sk file failed");

    // buffer write
    let mut file_generated = BufWriter::new(file_created);

    //  write file
    file_generated.write_all(sk.as_bytes()).expect("write failed");

    //println!("sk txt file generated.");

}

