use std::collections::HashMap;

use bollard::Docker;
use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::service::ImageSummary;

#[cfg(unix)]
fn get_client() -> Result<Docker, Error> {
    let client = Docker::connect_with_unix_defaults();
    client
}

//TODO send to UI thread https://github.com/daboross/screeps-rs/tree/master/network/src/tokio
pub async fn get_images() -> Result<Vec<ImageSummary>, Error> {
    let mut filters = HashMap::new();
    filters.insert("dangling", vec!["true"]);

    let options = Some(ListImagesOptions {
        all: true,
        filters,
        ..Default::default()
    });
    get_client()?.list_images(options).await
}