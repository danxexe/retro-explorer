use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PluginMeta {
    pub name: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
}

pub fn extract_plugin_meta(html: &str) -> Option<PluginMeta> {
    let re = Regex::new(r#"(?s)<script\s+type=["']application/plugin\+json["']\s*>(.*?)</script>"#)
        .ok()?;

    let caps = re.captures(html)?;
    let json = caps.get(1)?.as_str().trim();

    serde_json::from_str(json).ok()
}
