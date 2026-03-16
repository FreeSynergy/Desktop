use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotKind {
    Broadcast,
    Gatekeeper,
    Monitor,
    Digest,
    UserBot,
}

impl BotKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Broadcast  => "Broadcast",
            Self::Gatekeeper => "Gatekeeper",
            Self::Monitor    => "Monitor",
            Self::Digest     => "Digest",
            Self::UserBot    => "Personal Assistant",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Broadcast  => "📢",
            Self::Gatekeeper => "🔒",
            Self::Monitor    => "📊",
            Self::Digest     => "📋",
            Self::UserBot    => "🤖",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelTarget {
    pub platform: String,
    pub name: String,
    pub id: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BroadcastRecord {
    pub message: String,
    pub sent_at: DateTime<Utc>,
    pub target_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PendingApproval {
    pub id: String,
    pub username: String,
    pub platform: String,
    pub waiting_since: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagingBot {
    pub id: String,
    pub name: String,
    pub kind: BotKind,
    pub enabled: bool,
    pub targets: Vec<ChannelTarget>,
    #[serde(default)]
    pub recent_broadcasts: Vec<BroadcastRecord>,
    #[serde(skip)]
    pub pending_approvals: Vec<PendingApproval>,
}

impl MessagingBot {
    pub fn demo_bots() -> Vec<Self> {
        vec![
            Self {
                id: "broadcast".into(),
                name: "Broadcast Bot".into(),
                kind: BotKind::Broadcast,
                enabled: true,
                targets: vec![
                    ChannelTarget { platform: "Telegram".into(), name: "FreeSynergy Community".into(), id: "-100123".into(), enabled: true },
                    ChannelTarget { platform: "Matrix".into(),   name: "#general:example.com".into(),  id: "!abc:example.com".into(), enabled: true },
                ],
                recent_broadcasts: vec![
                    BroadcastRecord {
                        message: "Neues Update verfügbar…".into(),
                        sent_at: Utc::now() - chrono::Duration::hours(2),
                        target_count: 4,
                    },
                ],
                pending_approvals: vec![],
            },
            Self {
                id: "gatekeeper".into(),
                name: "Gatekeeper Bot".into(),
                kind: BotKind::Gatekeeper,
                enabled: true,
                targets: vec![
                    ChannelTarget { platform: "Telegram".into(), name: "FreeSynergy Community".into(), id: "-100123".into(), enabled: true },
                ],
                recent_broadcasts: vec![],
                pending_approvals: vec![
                    PendingApproval {
                        id: "1".into(),
                        username: "@alice_t".into(),
                        platform: "Telegram".into(),
                        waiting_since: Utc::now() - chrono::Duration::minutes(2),
                    },
                    PendingApproval {
                        id: "2".into(),
                        username: "@bob_99".into(),
                        platform: "Telegram".into(),
                        waiting_since: Utc::now() - chrono::Duration::minutes(4),
                    },
                ],
            },
        ]
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MessagingBotsConfig {
    #[serde(default)]
    pub bots: Vec<MessagingBot>,
}

impl MessagingBotsConfig {
    fn path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".config").join("fsn").join("messaging_bots.toml")
    }

    pub fn load() -> Vec<MessagingBot> {
        let path = Self::path();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let cfg: Self = toml::from_str(&content).unwrap_or_default();
        if cfg.bots.is_empty() { MessagingBot::demo_bots() } else { cfg.bots }
    }

    pub fn save(bots: &[MessagingBot]) -> Result<(), String> {
        let path = Self::path();
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
        }
        let cfg = Self { bots: bots.to_vec() };
        let content = toml::to_string_pretty(&cfg).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }
}
