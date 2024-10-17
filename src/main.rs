// pub async fn list_objects(client: &aws_sdk_s3::Client, bucket: &str) -> Result<(), ()> {
//     let mut response = client
//         .list_objects_v2()
//         .bucket(bucket.to_owned())
//         .max_keys(10) // In this example, go 10 at a time.
//         .into_paginator()
//         .send();

//     while let Some(result) = response.next().await {
//         match result {
//             Ok(output) => {
//                 for object in output.contents() {
//                     println!(" - {}", object.key().unwrap_or("Unknown"));
//                 }
//             }
//             Err(err) => {
//                 eprintln!("{err:?}")
//             }
//         }
//     }

//     Ok(())
// }
const REGION: &str = "eu-north-1";
const BUCKET_NAME: &str = "autoweatherstation";

use chrono::Local;
use reqwest::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::v2024_03_28())
        .region(REGION)
        .load()
        .await;
    let s3 = aws_sdk_s3::Client::new(&shared_config);

    // list_objects(&s3,"autoweatherstation").await.unwrap();

     // URL of the endpoint
     let url = "http://aws.imd.gov.in:8091/AWS/temp.php?a=60&b=ALL_STATE";
    
     // Get the current date and time
     let now = Local::now();
     // Format the date and time as a string
     let formatted_date_time = now.format("%Y%m%d_%H").to_string();
 
     // Path to save the file, including the date and time
     let file_path = format!("aws_{}.txt", formatted_date_time);
 
     // Fetch the response and save it to a file
     fetch_and_save(url, &file_path,&s3).await;
    Ok(())
}

async fn fetch_and_save(url: &str, file_path: &str,s3_client: &aws_sdk_s3::Client) -> Result<(), Error> {
    // Send a GET request to the specified URL
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Get the response body as bytes
        let content = response.bytes().await?;
        let mut file = File::create(file_path).unwrap();

        file.write_all(&content);
        println!("File saved to {}", file_path);
        let awsdata = tokio::fs::read(file_path).await.unwrap();

     let _ = s3_client
        .put_object()
        .bucket(BUCKET_NAME)
        // .storage_class(aws_sdk_s3::types::StorageClass::ReducedRedundancy)
        .key(file_path)
        .content_length(awsdata.len() as i64)
        .body(awsdata.into())
        .set_content_type(Some("text/plain".to_owned()))
        .send().await;
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

