use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct LANBroadcastConfig {
    pub enabled: bool,
    // We use an extra `motd` because this only supports one line,
    // but we use the server `motd` without new lines as the default.
    pub motd: Option<String>,
    // Allow users to specify port so that the port is predictable.
    // There are many reasons why the port might need to be predictable.
    // One reason is Docker containers, where specific ports need to be allowed.
    pub port: Option<u16>,
}
