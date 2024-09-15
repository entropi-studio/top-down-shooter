use crate::level::{LevelObject, SerializeLevelObject};

pub fn write_level(objects: Vec<LevelObject>) -> String {
    let mut content = String::new();
    for object in objects {
        let (statement, args) = object.serialize();
        let args = args
            .iter()
            .map(|a| {
                let url_encoded = url::form_urlencoded::byte_serialize(a.as_bytes());
                String::from_iter(url_encoded)
            })
            .collect::<Vec<_>>()
            .join(" ");
        content += format!("{statement} {args}\n").as_str();
    }

    content
}
