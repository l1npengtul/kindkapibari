use crate::manifest::CoconutPakManifest;
use crate::text::TextContainer;
use crate::version::Version;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoconutPakOutput {
    pub edition: Version,
    pub manifest: CoconutPakManifest,
    // files
    pub register_text_containers: Option<Vec<TextContainer>>,
}
