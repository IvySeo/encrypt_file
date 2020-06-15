/* --------------------- HOW IT WORKS ---------------------------
ENCRYPT THE FILE:
generate public key and secret key -> open file to encrypt -> encrypt the file and write a file and save the file name in UUID -> pass the file name (UUID) to aws s3 function 

UPLOAD THE FILE TO AMAZON S3 bucket: BUCKETNAME /holding folder:
-> open the file that was just encrypted and written and saved in local
-> upload the encrypted file to the amazon s3 -> once the file is uploaded to the s3 successfully, delete the file in local
*/

extern crate libsodium_sys as ffi;
extern crate std;
extern crate s3;

#[cfg(not(feature = "std"))]

mod write_file;
mod encrypt;
mod aws_s3;

extern crate libc;

pub fn main(){
    
    //start_loop();
    run_once();
}

pub fn run_once(){
    let pk = encrypt::generate_pk_sk();
    let encrypted = encrypt::encrypt_start(pk);
    println!("encrypted UUID: {:?}", encrypted);
    match encrypted{
        Ok(uuid) =>  aws_s3::aws_s3_func(uuid),
        Err(err) => println!("!Error to encrypt : {:?}", err)
    }
}

pub fn start_loop(){
    let mut count = 0u32;
    loop {
        count += 1;

            let pk = encrypt::generate_pk_sk();
            let encrypted = encrypt::encrypt_start(pk);
            println!("count: {} encrypted UUID: {:?}", count, encrypted);
            match encrypted{
                Ok(uuid) =>  aws_s3::aws_s3_func(uuid),
                Err(err) => println!("!Error to encrypt : {:?}", err)
            }

        if count == 120 {
            println!("count 120, break");
            // Exit this loop
            break;
        }
    }
}

