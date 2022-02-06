use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffirmPak {
    id: Uuid,
    name: String,
}
