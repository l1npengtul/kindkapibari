use crate::ESult;
use kindkapibari_core::manifest::CoconutPakManifest;

fn init(name: Option<String>, no_text: bool, registry: Option<String>) -> ESult<()> {
    let name = name.unwrap_or(std::env::current_dir()?.to_string_lossy().to_string());

    let mut manifest = CoconutPakManifest::default();
}
