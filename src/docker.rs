use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{Thread, yield_now};

use bollard::container::ListContainersOptions;
use bollard::Docker;
use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::service::{ContainerSummaryInner, ImageSummary};
use tokio::time::{Duration, Instant};

use crate::components::main_app::MainApp;

// TODO: could be memoized or static
#[cfg(unix)]
fn get_client() -> Result<Docker, Error> {
    let client = Docker::connect_with_unix_defaults();
    client
}

//TODO send to UI thread https://github.com/daboross/screeps-rs/tree/master/network/src/tokio
pub async fn get_images() -> Result<Vec<ImageSummary>, Error> {
    let mut filters: HashMap<&str, Vec<&str>, RandomState> = HashMap::new();
    // filters.insert("dangling", vec!["true"]);

    let options = Some(ListImagesOptions {
        all: true,
        filters,
        ..Default::default()
    });
    get_client()?.list_images(options).await
}

//TODO send to UI thread https://github.com/daboross/screeps-rs/tree/master/network/src/tokio
pub async fn get_containers() -> Result<Vec<ContainerSummaryInner>, Error> {
    let mut filters: HashMap<&str, Vec<&str>, RandomState> = HashMap::new();
    // filters.insert("dangling", vec!["true"]);
    // filters.insert("status", vec!["running"]);


    let options = Some(ListContainersOptions {
        all: false,
        filters,
        ..Default::default()
    });
    get_client()?.list_containers(options).await
}

#[derive(Debug)]
pub enum IOEvent {
    RefreshContainers,
    RefreshImages,
}

// Receive a message and handle it
#[tokio::main]
pub async fn start_tokio(app: &Mutex<MainApp>, io_rx: std::sync::mpsc::Receiver<IOEvent>) {
    while let Ok(event) = io_rx.recv() {
        log::info!("Received event in loop {:?}", event);
        match event {
            IOEvent::RefreshContainers => {
                let containers = get_containers().await;
                match containers {
                    Ok(containers) => {
                        let mut app = app.lock().unwrap();
                        log::info!("Containers: {:?}", containers);
                        app.containers = containers;
                    }
                    Err(err) => {
                        log::error!("There was an error retrieving containers, {}", err);
                    }
                }
            }
            IOEvent::RefreshImages => {
                let images = get_images().await;
                match images {
                    Ok(images) => {
                        let mut app = app.lock().unwrap();
                        log::info!("Images: {:?}", images.len());
                        app.images = images;
                    }
                    Err(err) => {
                        log::error!("There was an error retrieving images, {:?}", err);
                    }
                }
            }
        }
        tokio::time::delay_for(Duration::from_millis(100)).await;
    };
}