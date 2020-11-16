use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::{Arc};
use std::thread::{Thread, yield_now};

use bollard::container::ListContainersOptions;
use bollard::Docker;
use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::service::{ContainerSummaryInner, ImageSummary, VolumeListResponse};
use tokio::time::{Duration, Instant};

use crate::app::App;
use tokio::sync::Mutex;
use bollard::volume::ListVolumesOptions;
use crate::component;
use component::containers::Containers;
use crate::component::images::Images;
use crate::component::volumes::Volumes;

// TODO: could be memoized or static
#[cfg(unix)]
fn get_client() -> Result<Docker, Error> {
    let client = Docker::connect_with_unix_defaults();
    client
}

pub async fn get_images() -> Result<Vec<ImageSummary>, Error> {
    let filters: HashMap<&str, Vec<&str>, RandomState> = HashMap::new();
    // let mut filters: HashMap<&str, Vec<&str>, RandomState> = HashMap::new();
    // filters.insert("dangling", vec!["true"]);

    let options = Some(ListImagesOptions {
        all: true,
        filters,
        ..Default::default()
    });
    get_client()?.list_images(options).await
}

pub async fn get_containers() -> Result<Vec<ContainerSummaryInner>, Error> {
    let filters: HashMap<&str, Vec<&str>, RandomState> = HashMap::new();

    let options = Some(ListContainersOptions {
        all: false,
        filters,
        ..Default::default()
    });
    get_client()?.list_containers(options).await
}

pub async fn get_volumes() -> Result<VolumeListResponse, Error> {
    let filters: HashMap<&str, Vec<&str>, RandomState> = HashMap::new();

    let options = Some(ListVolumesOptions {
        filters,
        ..Default::default()
    });
    get_client()?.list_volumes(options).await
}

#[derive(Debug)]
pub enum IOEvent {
    RefreshContainers,
    RefreshImages,
    RefreshVolumes,
}

// Receive a message and handle it
#[tokio::main]
pub async fn start_tokio(app: &Arc<Mutex<App>>, io_rx: std::sync::mpsc::Receiver<IOEvent>) {
    while let Ok(event) = io_rx.recv() {
        log::debug!("Received event in loop {:?}", event);
        match event {
            //TODO these will change to then update the widgets data not the app
            IOEvent::RefreshContainers => {
                let containers = get_containers().await;
                match containers {
                    Ok(containers) => {
                        let mut app = app.lock().await;
                        log::debug!("Containers: {:?}", containers);
                        app.containers_widget = Some(Containers::new_with_items(containers))
                        // app.container_data = containers; TODO
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
                        let mut app = app.lock().await;
                        log::debug!("Images: {:?}", images);
                        app.images_widget = Some(Images::new_with_items(images));
                    }
                    Err(err) => {
                        log::error!("There was an error retrieving images, {:?}", err);
                    }
                }
            }
            IOEvent::RefreshVolumes => {
                let volumes = get_volumes().await;
                match volumes {
                    Ok(volumes) => {
                        let mut app = app.lock().await;
                        log::debug!("Volumes: {:?}", volumes);
                        app.volumes_widget = Some(Volumes::new_with_items(volumes));
                    }
                    Err(err) => {
                        log::error!("There was an error retrieving volumes, {:?}", err);
                    }
                }
            }
        }
    };
}