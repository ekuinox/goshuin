use std::sync::Arc;

use crate::client::GoshuinRepositoryClient;
use crate::facility::{Coordinate, Facility, FacilityKind, Goshuin};
use anyhow::{bail, Result};
use chrono::{Date, Utc};
use serenity::prelude::*;

/// ここに編集中のFacilityを保持する
pub struct Editor {
    client: GoshuinRepositoryClient,
    facility: Option<Facility>,
}

impl Editor {
    pub fn new(client: GoshuinRepositoryClient) -> Editor {
        Editor {
            client,
            facility: None,
        }
    }

    /// id を元に新規 Facility を探す
    pub async fn open(&mut self, id: &String) -> Result<&Facility> {
        let branch_name = format!("deploy-{}", id);
        let facility = self.client.get_facility(id, branch_name).await?;
        self.facility = Some(facility);
        let facility = self.facility.as_ref().unwrap();
        Ok(facility)
    }

    /// 新しく作成する
    pub fn create(
        &mut self,
        id: String,
        name: String,
        kind: FacilityKind,
        coordinate: Coordinate,
    ) -> &Facility {
        let facility = Facility::new(id, name, kind, coordinate, vec![], None, None);
        self.facility = Some(facility);
        self.facility.as_ref().unwrap()
    }

    /// 保持しているものを取り消す
    pub fn clear(&mut self) {
        self.facility = None;
    }

    /// 画像をコミットする
    pub async fn create_image(&self, origin_url: &String, name: &String) -> Result<()> {
        let facility = match &self.facility {
            Some(facility) => facility,
            None => bail!("facility is none"),
        };
        let branch_name = format!("deploy-{}", facility.id);

        let existed = self.client.is_existed_branch(branch_name.clone()).await?;
        if !existed {
            self.client.create_branch(branch_name.clone()).await?;
        }

        let _ = self.client.write_image(origin_url, name, branch_name.clone()).await;

        Ok(())
    }

    /// 御朱印を拡張する
    pub fn append_goshuin(
        &mut self,
        images: Vec<String>,
        date: String,
        desc: Option<String>,
    ) -> Result<()> {
        let facility = match &mut self.facility {
            Some(facility) => facility,
            None => bail!("facility is none"),
        };
        let images = images.into_iter().map(|name| format!("/images/{}", name)).collect::<Vec<String>>();
        let goshuin = Goshuin::new(images, date, desc);
        facility.goshuin_list.push(goshuin);
        Ok(())
    }

    /// GitHub に書き込む
    pub async fn write(&self) -> Result<()> {
        let facility = match &self.facility {
            Some(facility) => facility,
            None => bail!("facility is none"),
        };
        let branch_name = format!("deploy-{}", facility.id);

        let existed = self.client.is_existed_branch(branch_name.clone()).await?;
        if !existed {
            self.client.create_branch(branch_name.clone()).await?;
        }

        let _ = self
            .client
            .write_facility(facility, branch_name.clone())
            .await?;

        Ok(())
    }

    /// 保持している Facility を取得する
    pub fn get_facility(&self) -> Option<&Facility> {
        self.facility.as_ref()
    }
}

pub struct EditorData;

impl TypeMapKey for EditorData {
    type Value = Arc<Mutex<Editor>>;
}
