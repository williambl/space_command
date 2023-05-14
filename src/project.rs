use crate::track::Track;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub tracks: Vec<Track>
}
