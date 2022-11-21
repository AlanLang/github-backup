use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path, process::Command};
use tokio::time;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    clone_url: String,
    name: String,
}

#[tokio::main]
async fn main() {
    let token = get_env("GITHUB_TOKEN");
    fs::create_dir("./backup").unwrap_or_default();
    timer_to_back(&token).await;
}

async fn timer_to_back(token: &str) {
    let mut interval = time::interval(time::Duration::from_secs(24 * 60 * 60));
    loop {
        interval.tick().await;
        println!("Start backup");
        let repos = get_repos(&token).await.unwrap();
        for rep in &repos {
            println!("Start backup {}", rep.name);
            sync_repo(rep, &token);
        }
        println!("End backup");
    }
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("{} not set", key),
    }
}

async fn get_repos(token: &str) -> Option<Vec<Repo>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let text_response = client
        .get("https://api.github.com/user/repos?per_page=1000")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let resp: Vec<Repo> = serde_json::from_str(&text_response).unwrap();
    Some(resp)
}

fn sync_repo(repo: &Repo, token: &str) {
    println!("开始同步仓库{}", repo.name);
    let path_url = format!("./backup/{}", repo.name);
    if Path::new(&path_url).exists() {
        println!("仓库已存在, 准备更新");
        pull_repo(repo);
    } else {
        println!("仓库不存在, 准备下载");
        clone_repo(repo, token);
    }
}

fn clone_repo(repo: &Repo, token: &str) {
    let clone_url = &repo
        .clone_url
        .replace("https://", &format!("https://{}@", token));
    let output = Command::new("git")
        .arg("clone")
        .arg(clone_url)
        .arg(format!("./backup/{}", &repo.name))
        .output()
        .expect("执行异常，提示");
    let output_str = String::from_utf8_lossy(&output.stdout);
    print!("{}", output_str)
}

fn pull_repo(repo: &Repo) {
    let output = Command::new("git")
        .arg("-C")
        .arg(format!("./backup/{}", &repo.name))
        .arg("pull")
        .arg("--all")
        .output()
        .expect("执行异常，提示");
    let output_str = String::from_utf8_lossy(&output.stdout);
    print!("{}", output_str)
}
