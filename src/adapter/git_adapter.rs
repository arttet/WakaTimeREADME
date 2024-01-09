use crate::port::UploadContentPort;

use crate::application::config::GitConfig;

use async_trait::async_trait;
use color_eyre::eyre::{eyre, Ok, Result};

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct GitAdapter {
    cfg: GitConfig,
}

impl GitAdapter {
    pub fn new(cfg: GitConfig) -> Self {
        Self { cfg }
    }
}

#[async_trait]
impl UploadContentPort for GitAdapter {
    async fn upload_content(&self, metrics: String) -> Result<()> {
        let temp_dir = tempfile::Builder::new().prefix("wakatime_bot_").tempdir()?;
        let target_file_path = temp_dir.path().join(&self.cfg.file_path);

        let repo_path = temp_dir.path();

        self.download_repo(repo_path)?;
        self.insert_metrics(&target_file_path, &metrics)?;

        let has_changes = self.has_changes(repo_path)?;
        if has_changes {
            self.make_commit(repo_path)?;
            self.push_branch_to_remote(repo_path)?;
        }

        Ok(())
    }
}

impl GitAdapter {
    fn download_repo(&self, local_path: &Path) -> Result<()> {
        let mut callbacks = git2::RemoteCallbacks::new();

        let username = self.cfg.author_name.clone();
        let password = self.cfg.token.clone();
        callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            git2::Cred::userpass_plaintext(&username, &password)
        });

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let url = url::Url::parse(&self.cfg.base_url)?
            .join(&self.cfg.repository)?
            .to_string();

        let repo = builder.clone(&url, local_path)?;
        repo.find_remote("origin")?
            .fetch(&[&self.cfg.branch_name], None, None)?;

        Ok(())
    }

    fn insert_metrics(&self, file_path: &Path, metrics: &str) -> Result<()> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let begin_pos = content.find(&self.cfg.begin_block).ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Begin block not found",
        ))? + self.cfg.begin_block.len();

        let end_pos = content.rfind(&self.cfg.end_block).ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "End block not found",
        ))?;

        if begin_pos < end_pos {
            content.replace_range(begin_pos..end_pos, metrics);
        } else {
            return Err(eyre!("Invalid positions for replacement"));
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    fn has_changes(&self, repo_path: &Path) -> Result<bool> {
        let repo = git2::Repository::open(repo_path)?;

        let statuses = repo.statuses(None)?;
        let has_changes = statuses
            .iter()
            .any(|entry| entry.status() != git2::Status::CURRENT);

        Ok(has_changes)
    }

    fn make_commit(&self, repo_path: &Path) -> Result<()> {
        let repo = git2::Repository::open(repo_path)?;

        let mut index = repo.index()?;
        index.add_path(Path::new(&self.cfg.file_path))?;
        index.write()?;

        let new_tree_oid = index.write_tree()?;
        let new_tree = repo.find_tree(new_tree_oid)?;

        let author_signature = git2::Signature::now(&self.cfg.author_name, &self.cfg.author_email)?;
        let committer_signature =
            git2::Signature::now(&self.cfg.committer_name, &self.cfg.committer_email)?;
        let commit_message = &self.cfg.commit_message;

        let head = repo.head().unwrap();
        let parent = repo.find_commit(head.target().unwrap()).unwrap();

        repo.commit(
            Some("HEAD"),
            &author_signature,
            &committer_signature,
            commit_message,
            &new_tree,
            &[&parent],
        )?;

        Ok(())
    }

    fn push_branch_to_remote(&self, repo_path: &Path) -> Result<()> {
        let repo = git2::Repository::open(repo_path)?;

        let username = self.cfg.author_name.clone();
        let password = self.cfg.token.clone();

        let mut remote_callbacks = git2::RemoteCallbacks::new();
        remote_callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            git2::Cred::userpass_plaintext(&username, &password)
        });

        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(remote_callbacks);

        let local_ref = format!("refs/heads/{}", &self.cfg.branch_name);

        let mut remote = repo.find_remote("origin")?;
        remote.push(&[&local_ref], Some(&mut push_options))?;

        Ok(())
    }
}
