use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use futures::stream;
use tokio::fs;
use serde_json::Value;
use tokio::runtime::Runtime;
mod AzureApi;


use futures::stream::StreamExt; // Import the StreamExt trait
use crate::AzureApi::{MyRequest, MyResponse};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = "/Users/bogdankoval/Downloads/photos/";

    // Read the directory
    let mut dir_entries = fs::read_dir(dir).await?;

    // Convert the ReadDir to a Stream
    let dir_entries_stream = stream::unfold(dir_entries, |mut dir_entries| async {
        match dir_entries.next_entry().await {
            Ok(Some(entry)) => Some((entry, dir_entries)),
            _ => None,
        }
    });

    // Process each entry concurrently
    dir_entries_stream.for_each_concurrent(None, |entry| async move {
        let path = entry.path();
        if path.is_file() {
            // Create a new request for each file
            let mut request = MyRequest::new("4d7bd39a70c249eebd19f5b8d62f5d7b", vec!["tags", "caption"]);
            request.set_img(path.to_str().unwrap()).unwrap();

            let response= request.send_request().await.unwrap();
            let response_copy = response.json::<Value>().await.unwrap();
            let response_struct: MyResponse =response_copy.clone().into();
            println!("{:?}", response_struct.caption);
        }
    }).await;

    Ok(())
}