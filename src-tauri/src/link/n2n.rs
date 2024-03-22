pub mod edge;

#[allow(dead_code)]
pub const N2N_VERSION: &str = "3.1.1";

/// 来源：https://github.com/ntop/n2n/blob/3.1.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmd {
    // ReloadCommunities,
    Stop, // 停止
    Verbose,
    Communities,
    Edges,
    Supernodes,
    Timestamps,
    PacketStats,
    // PostTest,
    // Help,
    // HelpEvents,
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
            // Cmd::PostTest => "post.test",
            // Cmd::Help => "help",
            // Cmd::HelpEvents => "help.events",
        }
    }
}

pub enum Action {
    Write,
    Read,
}

impl Action {
    pub fn to_str(&self) -> &str {
        match self {
            Action::Write => "w",
            Action::Read => "r",
        }
    }
}
