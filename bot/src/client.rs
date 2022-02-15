use crate::facility::Facility;
use anyhow::{bail, Result};
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

    pub async fn get_facility(&self, id: &String) -> Result<Facility> {
        let refs = self.get_repo().get_ref(&self.reference).await?;

        let sha = match refs.object {
            Object::Commit { sha, .. } => sha,
            Object::Tag { sha, .. } => sha,
            _ => bail!("err"),
        };

        let path = format!("facilities/{}.json", id);

        let content = self.get_repo().get_content().path(path).r#ref(sha).send().await?;
        let facility = match content
            .items
            .first()
            .and_then(|content| content.content.as_ref())
        {
            Some(content) => {
                let content = decode_content(&content)?;
                serde_json::from_str::<Facility>(&content)?
            },
            None => bail!("none"),
        };

        Ok(facility)
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
