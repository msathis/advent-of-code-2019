use tokio::prelude::*;
use tokio::fs::File;
use async_trait::async_trait;

#[async_trait]
pub trait Solution {
    async fn part1() -> String;
    async fn part2() -> String;
}

pub struct Input {
    pub path: String,
}

impl Input {
    pub fn new(path: String) -> Self {
        Input { path }
    }

    pub async fn read(&self) -> Vec<String> {
        let mut file = match File::open(&self.path).await {
            Err(e) => panic!("File is wrong, {}", e),
            Ok(f) => f,
        };

        let mut buf = String::new();
        file.read_to_string(&mut buf).await;

        buf.lines().map(|d| d.to_string()).collect()
    }
}
