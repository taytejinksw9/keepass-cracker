pub mod keepass {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct KeePassEntry {
        pub title: String,
        pub username: String,
        pub password: String,
    }

    pub fn parse_entries(data: &[u8]) -> Vec<KeePassEntry> {
        serde_json::from_slice(data).expect("Failed to parse entries")
    }
}