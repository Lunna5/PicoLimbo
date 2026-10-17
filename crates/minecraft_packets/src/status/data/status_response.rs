use minecraft_protocol::prelude::ProtocolVersion;
use pico_text_component::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSample {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<PlayerSample>>,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: Component,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(
        alias = "enforcesSecureChat",
        default = "get_default_enforces_secure_chat"
    )]
    pub enforces_secure_chat: bool,
}

fn get_default_enforces_secure_chat() -> bool {
    false
}

impl StatusResponse {
    pub fn new(
        version_name: String,
        version_protocol: i32,
        description: &Component,
        online_players: u32,
        max_players: u32,
        favicon: Option<String>,
        protocol_version: ProtocolVersion,
    ) -> Self {
        let description = if protocol_version.is_after_inclusive(ProtocolVersion::V1_16)
            || protocol_version.is_any()
        {
            description.clone()
        } else {
            description.to_fallback()
        };
        StatusResponse {
            version: Version {
                name: version_name,
                protocol: version_protocol,
            },
            players: Players {
                max: max_players,
                online: online_players,
                sample: None,
            },
            description,
            favicon,
            enforces_secure_chat: false,
        }
    }
}
