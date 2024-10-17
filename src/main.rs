// fn main() {
//     println!("Hello, world!");
// }
pub async fn list_objects(client: &aws_sdk_s3::Client, bucket: &str) -> Result<(), ()> {
    let mut response = client
        .list_objects_v2()
        .bucket(bucket.to_owned())
        .max_keys(10) // In this example, go 10 at a time.
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(output) => {
                for object in output.contents() {
                    println!(" - {}", object.key().unwrap_or("Unknown"));
                }
            }
            Err(err) => {
                eprintln!("{err:?}")
            }
        }
    }

    Ok(())
}
const REGION: &str = "eu-north-1";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::v2024_03_28())
        .region(REGION)
        .load()
        .await;
    let s3 = aws_sdk_s3::Client::new(&shared_config);

    list_objects(&s3,"").await.unwrap();
    Ok(())
}