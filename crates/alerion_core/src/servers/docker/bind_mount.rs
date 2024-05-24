use std::io;
use std::path::PathBuf;

use bollard::models;
use bollard::secret::MountBindOptions;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use uuid::Uuid;

use crate::os::{DataDirectory, DataDirectoryImpl};

pub struct BindMount {
    path: PathBuf,
}

impl BindMount {
    /// Creates/resets a bind mount.
    pub async fn new(uuid: Uuid) -> io::Result<BindMount> {
        let mounts = DataDirectory::mounts();
        let path = mounts.create_clean(uuid).await?;
        std::fs::create_dir_all(&path)?;

        Ok(BindMount { path })
    }

    /// Remove everything in the bind mount folder.
    pub async fn clean(&self) -> io::Result<()> {
        let mut read_dir = tokio::fs::read_dir(&self.path).await?;

        let mut futures = FuturesUnordered::new();

        loop {
            let result = read_dir.next_entry().await;
            let Some(e) = result? else {
                break;
            };
            let rm_fut = tokio::fs::remove_file(e.path());
            futures.push(rm_fut);
        }

        while let Some(r) = futures.next().await {
            r?;
        }

        Ok(())
    }

    pub fn to_docker_mount(&self, target: String) -> models::Mount {
        models::Mount {
            target: Some(target),
            source: Some(self.path.to_string_lossy().into_owned()),
            typ: Some(models::MountTypeEnum::BIND),
            read_only: Some(false),
            consistency: None,
            bind_options: Some(MountBindOptions {
                propagation: None,
                non_recursive: Some(true),
                create_mountpoint: None,
                read_only_non_recursive: None,
                read_only_force_recursive: None,
            }),
            volume_options: None,
            tmpfs_options: None,
        }
    }
}
