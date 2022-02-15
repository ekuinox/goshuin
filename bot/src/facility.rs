use serde::{Serialize, Deserialize};
use chrono::{Date, Utc};

/// 宗教施設の種別
#[derive(Serialize, Deserialize, Debug)]
pub enum FacilityKind {
    /// 寺院
    #[serde(rename = "temple")]
    Temple,

    /// 神社
    #[serde(rename = "shrine")]
    Shrine,
}

/// 座標
#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    /// 緯度
    lat: f64,

    /// 経度
    lon: f64,
}

impl Coordinate {
    pub fn new(lat: f64, lon: f64) -> Coordinate {
        Coordinate { lat, lon }
    }
}

/// 御朱印
#[derive(Serialize, Deserialize, Debug)]
pub struct Goshuin {
    /// 画像のURL
    #[serde(rename = "pictureUrls")]
    picture_urls: Vec<String>,
    
    /// 説明 メモとか
    description: Option<String>,

    /// 日付
    date: String,
}

impl Goshuin {
    pub fn new(
        picture_urls: Vec<String>,
        date: Date<Utc>,
        description: Option<String>
    ) -> Goshuin {
        Goshuin {
            description,            
            picture_urls,
            date: date.to_string(),
        }
    }
}

/// 付属物
#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    /// 画像のURL
    #[serde(rename = "mediaUrl")]
    media_url: String,

    /// 日付
    date: String,
}

impl Attachment {
    pub fn new(
        media_url: String,
        date: Date<Utc>
    ) -> Attachment {
        Attachment {
            media_url,
            date: date.to_string(),
        }
    }
}

/// 宗教施設情報
#[derive(Serialize, Deserialize, Debug)]
pub struct Facility {
    /// id
    id: String,

    /// 施設の名前
    name: String,

    /// 種別
    kind: FacilityKind,

    /// 座標
    coordinate: Coordinate,

    /// 御朱印のリスト
    #[serde(rename = "goshuinList")]
    goshuin_list: Vec<Goshuin>,

    /// なんかメモとか
    memo: Option<String>,

    /// 付属物
    attachments: Option<Vec<Attachment>>,
}

impl Facility {
    pub fn new(
        id: String,
        name: String,
        kind: FacilityKind,
        coordinate: Coordinate,
        goshuin_list: Vec<Goshuin>,
        memo: Option<String>,
        attachments: Option<Vec<Attachment>>
    ) -> Facility {
        Facility {
            id,
            name,
            kind,
            memo,
            coordinate,
            attachments,
            goshuin_list,
        }
    }
}

#[test]
fn test_parse() {
    let json = r#"
    {
        "id": "hirose-taisha",
        "name": "廣瀬大社",
        "kind": "shrine",
        "coordinate": {
            "lat": 34.5913983,
            "lon": 135.7483875
        },
        "goshuinList": [
            {
                "pictureUrls": ["/images/IMG_6327.jpg"],
                "description": "",
                "date": "2021-10-24"
            }
        ]
    }
    "#;

    let facility = serde_json::from_str::<Facility>(&json);
    dbg!(&facility);

    assert!(facility.is_ok());
}
