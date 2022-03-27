use kindkapybari_core::manifest::CoconutPakManifest;
use kindkapybari_core::text::TextContainer;
use semver::Version;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoconutPakOutput {
    pub edition: Version,
    pub manifest: CoconutPakManifest,
    // files
    pub register_text_containers: Option<Vec<TextContainer>>,
}
