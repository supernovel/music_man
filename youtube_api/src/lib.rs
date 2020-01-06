use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_test::assert_ok;

const API_DOAMIN: &'static str = "https://www.googleapis.com/youtube/v3";
const PLAYLIST_ITEMS_PATH: &'static str = "/playlistItems";
const PLAYLIST_PATH: &'static str = "/playlistItems";

/// 아이템 아이디
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    pub kind: String,
    pub video_id: Option<String>,
    pub channel_id: Option<String>,
    pub playlist_id: Option<String>,
}

/// 아이템 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    published_at: Option<DateTime<Utc>>,
    channel_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    thumbnails: Option<Thumbnails>,
    channel_title: Option<String>,
    live_broadcast_content: Option<String>,
    playlist_id: Option<String>,
    position: Option<u32>,
    resource_id: Option<Id>,
}

/// 썸네일 리스트
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    default: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    high: Option<Thumbnail>,
    standard: Option<Thumbnail>,
    maxres: Option<Thumbnail>,
}

/// 썸네일 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    url: String,
    width: Option<u64>,
    height: Option<u64>,
}

/// 플레이리스트 아이템
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    kind: String,
    id: Option<String>,
    snippet: Option<Snippet>,
}

/// 플레이리스트 아이템 리스트 페이징
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemPage {
    kind: String,
    next_page_token: String,
    prev_page_token: String,
    page_info: PlaylistItemPageInfo,
    items: Vec<PlaylistItem>,
}

/// 플레이리스트 아이템 리스트 페이징 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemPageInfo {
    total_results: i32,
    results_per_page: i32,
}

pub struct YMusicOptions {
    key: String,
}

pub struct YMusicController {
    playlistId: Option<String>,
    client: reqwest::Client,
    options: YMusicOptions,
}

impl YMusicController {
    pub fn new(key: String) -> YMusicController {
        Self {
            playlistId: None,
            client: reqwest::Client::new(),
            options: YMusicOptions { key },
        }
    }

    pub fn search(&self, keyword: String) -> Vec<String> {
        vec![keyword]
    }

    pub async fn add(
        &self,
        id: impl Into<String>,
    ) -> Result<PlaylistItem, Box<dyn std::error::Error>> {
        let item = PlaylistItem {
            kind: String::from("youtube#playlistItem"),
            id: None,
            snippet: Option::Some(Snippet {
                playlist_id: self.playlistId.clone(),
                resource_id: Option::Some(Id {
                    kind: String::from("youtube#video"),
                    video_id: Option::Some(id.into()),
                    channel_id: None,
                    playlist_id: None,
                }),
                position: None,
                published_at: None,
                channel_id: None,
                title: None,
                description: None,
                thumbnails: None,
                channel_title: None,
                live_broadcast_content: None,
            }),
        };

        let result = self
            .client
            .post([API_DOAMIN, PLAYLIST_ITEMS_PATH].join("").as_str())
            .bearer_auth(self.options.key.as_str())
            .query(&["part", "snippet"])
            .json(&item)
            .send()
            .await?
            .json::<PlaylistItem>()
            .await?;

        Result::Ok(result)
    }

    pub async fn get_list(&self) -> Result<PlaylistItemPage, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get([API_DOAMIN, PLAYLIST_ITEMS_PATH].join("").as_str())
            .bearer_auth(self.options.key.as_str())
            .query(&[
                ("part", "snippet"),
                ("playlistId", self.playlistId.clone().unwrap().as_str()),
            ])
            .send()
            .await?
            .json::<PlaylistItemPage>()
            .await?;

        Result::Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn music_search() {
        let controller = YMusicController::new(String::from(""));
        assert_eq!(vec!["1234"], controller.search(String::from("1234")));
    }

    #[tokio::test]
    async fn music_add() {
        let controller = YMusicController::new(String::from(""));
        assert_ok!(controller.add("jLUa6brtC-c").await);
    }

    #[tokio::test]
    async fn get_music_list() {
        let controller = YMusicController::new(String::from(""));
        assert_ok!(controller.get_list().await);
    }
}
