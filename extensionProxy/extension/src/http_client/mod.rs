use std::sync::OnceLock;

pub static CELL: OnceLock<reqwest::Client> = OnceLock::new();

pub fn client() -> &'static reqwest::Client {
    CELL.get_or_init(|| reqwest::Client::new())
}
