use clap::Parser;

// in Byte
pub const PACKET_SIZE: usize = 10;
// 100ms
pub const COUNT_GRAIN_PRE_SEC: u32 = 10;

#[derive(Parser, Debug, Clone, PartialEq)]
#[clap(version = "1.0", author = "ihciah <ihciah@gmail.com>")]
pub struct ServerConfig {
    #[clap(
        short,
        long,
        min_values = 1,
        default_value = "1",
        about = "cpu core id list"
    )]
    pub cores: Vec<u8>,
    #[clap(
        short,
        long,
        about = "bind address, like 127.0.0.1:8080",
        default_value = "[::]:40000"
    )]
    pub bind: String,
}

#[derive(Parser, Debug, Clone, PartialEq)]
#[clap(version = "1.0", author = "ihciah <ihciah@gmail.com>")]
pub struct ClientConfig {
    #[clap(
        short,
        long,
        min_values = 1,
        default_value = "0",
        about = "cpu core id list"
    )]
    pub cores: Vec<u8>,
    #[clap(
        short = 'n',
        long,
        about = "connection numbers per core",
        default_value = "50"
    )]
    pub conns_per_core: usize,
    #[clap(
        short,
        long,
        about = "QPS limit per core, leave blank means unlimited"
    )]
    pub qps_per_core: Option<usize>,
    #[clap(
        short,
        long,
        about = "target address, like 127.0.0.1:8080",
        default_value = "127.0.0.1:40000"
    )]
    pub target: String,
}

impl ServerConfig {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

impl ClientConfig {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

pub fn format_cores(cores: &[u8]) -> String {
    cores
        .iter()
        .map(|&c| c.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg() {
        let cfg = ServerConfig::parse_from(&["test", "-c", "1", "2", "-b", ":8080"]);
        assert_eq!(
            cfg,
            ServerConfig {
                cores: vec![1, 2],
                bind: ":8080".to_string()
            }
        );

        let cfg = ServerConfig::parse_from(&["test", "-b", ":8080"]);
        assert_eq!(
            cfg,
            ServerConfig {
                cores: vec![1],
                bind: ":8080".to_string()
            }
        );
    }
}
