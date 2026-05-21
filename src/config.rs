use serde;
use toml;

struct config {
    layout: String,
    icon_theme: Option<String>,
}
