use reqwest::Url;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tokio::sync::mpsc;

pub struct DownloadTask {
    pub url: Url,
    pub file_path: Path,
}

pub struct Downloader {
    tasks: Vec<DownloadTask>,
}

impl Downloader {
    pub fn new(tasks: Vec<DownloadTask>) -> Self {
        Self { tasks }
    }

    pub async fn start<F: Fn(usize) + Send + 'static>(&self, progress_callback: F) {
        let (tx, mut rx) = mpsc::channel(32);

        for (index, task) in self.tasks.iter().enumerate() {
            let tx_clone = tx.clone();

            tokio::spawn(async move {
                let res = reqwest::get(task.url.clone()).await.unwrap();
                let bytes = res.bytes().await.unwrap();
                let mut file = File::create(&task.file_path).unwrap();
                file.write_all(&bytes).unwrap();
                tx_clone.send(index).await.unwrap();
            });
        }

        while let Some(progress) = rx.recv().await {
            progress_callback(progress);
        }
    }
}