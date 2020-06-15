use std::fs::File;
use std::io::Read;
use s3::bucket::Bucket;
use s3::credentials::Credentials;
use std::fs;
pub const STATUS_CODE: u32 = 200;


pub fn aws_s3_func(file_name_uuid: String) {
    println!("aws_s3_func - passed uuid: {:?}", file_name_uuid);

    let access_key_ = "YOUR ACCESS KEY";
    let secret_key_ = "YOUR SECRET KEY";
    let bucket_name = "YOUR BUCKET NAME";
    let region = "us-east-2".parse().unwrap();

    let credentials = Credentials::new(access_key_, secret_key_, None);
    let bucket = Bucket::new(bucket_name, region, credentials);

    // attempts to open a file in read-only mode
    // File::open(..) is used to open a file in reading mode only 
    // then read_to_end() reads the content of a file to the end and stored in a vector content
    let mut file:File = File::open(format!("C:/Users/M7/Desktop/encrypt_file/encrypted_files/{}",file_name_uuid)).unwrap();
    let mut content: Vec<u8> = vec![];

    // read all bytes until EOF in this source, placing them into  'buf'
    file.read_to_end(&mut content).unwrap();
    println!("file read successfully");

    put_file(bucket, content, file_name_uuid);
}

pub fn put_file(bucket: Bucket, content: Vec<u8>, file_name_uuid: String){

    let filepath_to_upload_to_s3 = format!("/holding/{}", file_name_uuid);
    println!("filepath_to_upload_to_s3: {:?}", filepath_to_upload_to_s3);

    // put() method is used to put the content in the bucket
    let (_, code) = bucket.put(&filepath_to_upload_to_s3, &content, "text/plain" ).unwrap();

    assert_eq!(STATUS_CODE, code);

    println!("STATUS_CODE: {:?}, code: {:?}", STATUS_CODE, code);
    println!("file uploaded in s3 BUCKETNAME successfully");

    // delete local encrypted file once it's uploaded in /holding folder
    if STATUS_CODE == code {
       let deleted = delete_local_encrypted_file(file_name_uuid);
       println!("file in local deleted: {:?}", deleted);
    }
}


pub fn delete_local_encrypted_file(file_name_uuid: String) ->  std::io::Result<()> {
    let path_to_delete_in_local = format!("C:/Users/M7/Desktop/encrypt_file/encrypted_files/{}", file_name_uuid);
    // delete file
    fs::remove_file(&path_to_delete_in_local)?;
    println!("delete file path: {:?}", path_to_delete_in_local);

    Ok(())

}

/* ----- THIS WILL LIST FILES AND UPLOAD LOCAL FILES TO THE BUCKET USING COMMAND LINE,
** IT WORKS BUT THE REASON WE USE ABOVE CODE IS TO MAKE SURE THE FILE IS UPLOADED USING S3 COMMAND.


use std::process::Command;

    pub fn aws_s3(){
           // list_files_in_holding();
            upload_encrypted_file_to_holding();
            list_files_in_holding();
    }

    pub fn list_files_in_holding(){
        println!("list_files_in_holding\n\n");

        // command : aws s3 ls s3://BUCKETNAME/holding/
        Command::new("aws")
        .arg("s3") // separates the command by space
        .arg("ls")
        .arg("s3://BUCKETNAME/holding/")
        .spawn() // runs the command
        .expect("aws s3 failed to list files in the holding folder"); // error msg
    }

    pub fn upload_encrypted_file_to_holding(){
        // upload all the files in the encrypted_file folder to the holding folder
        println!("\n\nupload_encrypted_file_to_holding\n\n");

        // command : aws s3 sync C:/Users/M7/Desktop/encrypt_file/encrypted_files s3://BUCKETNAME/holding/
        Command::new("aws")
        .arg("s3") // separates the command by space
        .arg("sync")
        .arg("C:/Users/M7/Desktop/encrypt_file/encrypted_files")
        .arg("s3://BUCKETNAME/holding/")
        .spawn() // runs the command
        .expect("aws s3 failed to upload an encrypted file to the holding folder"); // error msg
    }
   */
