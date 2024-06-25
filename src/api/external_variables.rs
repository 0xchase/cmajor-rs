use serde_json::{json, Value};

pub struct ExternalVariable {
    name: String,
    ty: Value,
    annotation: Value
}

impl ExternalVariable {
    pub fn from_json(json: &Value) -> Self {
        let name = json
            .get("name")
            .unwrap()
            .to_string();

        Self {
            name: json["name"]
                .as_str()
                .unwrap()
                .to_owned(),
            ty: json["type"].clone(),
            annotation: json["annotation"].clone()
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "type": self.ty,
            "annotation": self.annotation
        })
    }
}

pub fn create_audio_file_object(frames: &Value, sample_rate: f64) -> Value {
    todo!()
}
