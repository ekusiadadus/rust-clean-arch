use anyhow::Error;
use aws_sdk_s3::operation::create_bucket::{CreateBucketError, CreateBucketOutput};
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::{error::SdkError, Client};
use std::str;

async fn create_bucket(
    client: &Client,
    bucket_name: &str,
    region: &str,
) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await
}

async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<(), Error> {
    client.delete_bucket().bucket(bucket_name).send().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let bucket_name = "rust-sdk-test-bucket1";
    let region = "ap-northeast-1";

    let result = create_bucket(&client, bucket_name, region).await;
    match result {
        Ok(output) => println!("Bucket location: {:?}", output.location()),
        Err(error) => println!("Error: {:?}", error),
    }

    let result = delete_bucket(&client, bucket_name).await;
    match result {
        Ok(_) => println!("Bucket deleted"),
        Err(error) => println!("Error: {:?}", error),
    }
}
