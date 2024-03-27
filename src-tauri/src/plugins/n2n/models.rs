use serde::{Deserialize, Serialize};

pub mod args;
pub mod edge;
pub mod supernode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmd {
    // ReloadCommunities,
    Stop,
    Verbose,
    Communities,
    Edges,
    Supernodes,
    Timestamps,
    PacketStats,
}

impl Cmd {
    pub fn to_str(&self) -> &str {
        match self {
            // Cmd::ReloadCommunities => "reload_communities",
            Cmd::Stop => "stop",
            Cmd::Verbose => "verbose",
            Cmd::Communities => "communities",
            Cmd::Edges => "edges",
            Cmd::Supernodes => "supernodes",
            Cmd::Timestamps => "timestamps",
            Cmd::PacketStats => "packetstats",
        }
    }
}

pub enum Action {
    Write,
    Read,
    Sub,
}

impl Action {
    pub fn to_str(&self) -> &str {
        match self {
            Action::Write => "w",
            Action::Read => "r",
            Action::Sub => "s",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "_type", rename_all = "lowercase")]
pub enum Resp {
    Begin { cmd: String },
    Row {},
    End { cmd: String },
    Error { error: String },
}
