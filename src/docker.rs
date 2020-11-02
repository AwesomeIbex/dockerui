use std::collections::HashMap;

use bollard::Docker;
use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::service::{ImageSummary, ContainerSummaryInner};
use bollard::container::ListContainersOptions;
use std::sync::{Arc, Mutex};
use crate::components::main_app::MainApp;

// TODO: could be memoized or static
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

//TODO send to UI thread https://github.com/daboross/screeps-rs/tree/master/network/src/tokio
pub async fn get_containers() -> Result<Vec<ContainerSummaryInner>, Error> {
    let mut filters = HashMap::new();
    filters.insert("dangling", vec!["true"]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });
    get_client()?.list_containers(options).await
}

pub enum IOEvent {
    RefreshContainers,
    RefreshImages
}
// Receive a message and handle it
#[tokio::main]
pub async fn start_tokio(app: &Mutex<MainApp>, io_rx: std::sync::mpsc::Receiver<IOEvent>) {
    while let Ok(event) = io_rx.recv() {
        match event {
            IOEvent::RefreshContainers => {
                let containers = get_containers().await;
                match containers {
                    Ok(containers) => {
                        let mut app = app.lock().unwrap();
                        app.containers = containers
                    }
                    Err(_) => {}
                }
            }
            IOEvent::RefreshImages => {
                let images = get_images().await;
                match images {
                    Ok(images) => {
                        let mut app = app.lock().unwrap();
                        app.images = images
                    }
                    Err(_) => {}
                }
            }
        }
    }
}