use crate::facility::{Facility, Goshuin};
use anyhow::{bail, Result};
use chrono::{Date, Utc};
use octocrab::{models::repos::Object, params::repos::Reference, repos::RepoHandler, Octocrab};

pub struct GoshuinRepositoryClient {
    octocrab: Octocrab,
    owner: String,
    repo: String,
    reference: Reference,
}

impl GoshuinRepositoryClient {
    fn get_repo(&self) -> RepoHandler {
        self.octocrab.repos(&self.owner, &self.repo)
    }

    pub fn new(
        octocrab: Octocrab,
        owner: String,
        repo: String,
        branch: String,
    ) -> GoshuinRepositoryClient {
        GoshuinRepositoryClient {
            octocrab,
            owner,
            repo,
            reference: Reference::Branch(branch.into()),
        }
    }

    pub async fn get_facility(&self, id: &String, branch_name: String) -> Result<Facility> {
        let reference = if self.is_existed_branch(branch_name.clone()).await? {
            Reference::Branch(branch_name)
        } else {
            self.reference.to_owned()
        };
        let refs = self.get_repo().get_ref(&reference).await?;

        let sha = match refs.object {
            Object::Commit { sha, .. } => sha,
            Object::Tag { sha, .. } => sha,
            _ => bail!("err"),
        };

        let path = format!("facilities/{}.json", id);

        let content = self
            .get_repo()
            .get_content()
            .path(path)
            .r#ref(sha)
            .send()
            .await?;
        let facility = match content
            .items
            .first()
            .and_then(|content| content.content.as_ref())
        {
            Some(content) => {
                let content = decode_content(&content)?;
                serde_json::from_str::<Facility>(&content)?
            }
            None => bail!("none"),
        };

        Ok(facility)
    }

    /// 新しいブランチを作成する
    pub async fn create_branch(&self, name: String) -> Result<()> {
        let refs = self.get_repo().get_ref(&self.reference).await?;
        let sha = match refs.object {
            Object::Commit { sha, .. } => sha,
            Object::Tag { sha, .. } => sha,
            _ => bail!("err"),
        };
        let _ = self
            .get_repo()
            .create_ref(&Reference::Branch(name), sha)
            .await?;
        Ok(())
    }

    pub async fn write_image(
        &self,
        origin_url: &String,
        name: &String,
        branch: String,
    ) -> Result<()> {
        let path = format!("public/images/{}", name);
        let content = reqwest::get(origin_url)
            .await?
            .bytes()
            .await?
            .into_iter()
            .collect::<Vec<u8>>();

        let _ = self
            .get_repo()
            .create_file(path, format!("Add {}", name), content)
            .branch(branch)
            .send()
            .await?;
        Ok(())
    }

    /// 新しくファイルを追加する
    pub async fn write_facility(&self, facility: &Facility, branch: String) -> Result<()> {
        let reference = Reference::Branch(branch.clone());
        let refs = self.get_repo().get_ref(&reference).await?;

        let sha = match refs.object {
            Object::Commit { sha, .. } => sha,
            Object::Tag { sha, .. } => sha,
            _ => bail!("err"),
        };

        let path = format!("facilities/{}.json", facility.id);

        let sha = self
            .get_repo()
            .get_content()
            .path(path)
            .r#ref(sha)
            .send()
            .await
            .ok()
            .and_then(|c| c.items.first().map(|c| c.sha.to_string()));

        let path = format!("facilities/{}.json", facility.id);
        let content = serde_json::to_vec(&facility)?;
        let message = format!("Update {}", facility.id);
        if let Some(sha) = sha {
            let _ = self
                .get_repo()
                .update_file(path, message, content, sha)
                .branch(branch)
                .send()
                .await?;
        } else {
            let _ = self
                .get_repo()
                .create_file(path, message, content)
                .branch(branch)
                .send()
                .await?;
        }
        Ok(())
    }

    /// ブランチが存在するか
    pub async fn is_existed_branch(&self, name: String) -> Result<bool> {
        let r#ref = self.get_repo().get_ref(&Reference::Branch(name)).await;
        Ok(r#ref.is_ok()) // よくないこれ
    }
}

fn decode_content(c: &String) -> Result<String> {
    // 改行コードが 60 文字区切りで入っているので消していく
    let c = c
        .chars()
        .into_iter()
        .filter(|c| *c != '\n')
        .collect::<String>();
    let decoded = base64::decode(c)?;
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
