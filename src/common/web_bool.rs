use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct WebBool(pub bool);

impl WebBool {
    pub fn to_bool(&self) -> bool {
        self.0
    }
}
