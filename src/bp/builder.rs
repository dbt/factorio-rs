use base64::write::EncoderWriter;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::ser::StdError as Error;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Serialize)]
struct Entity {
    entity_number: u32,
    name: &'static str,
    position: Position,
}

#[derive(Debug, Serialize)]
struct Signal {
    #[serde(rename = "type")] // <-- this is also a container attribute
    type_: &'static str,
    name: &'static str,
}

#[derive(Debug, Serialize)]
struct Icon {
    signal: Signal,
    index: u8,
}

#[derive(Debug, Serialize)]
pub struct Builder {
    icons: Vec<Icon>,
    entities: Vec<Entity>,
    item: &'static str,
    label: String,
    description: String,
}

impl Builder {
    pub fn new<S1, S2>(label: S1, description: S2) -> Builder
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        return Builder {
            icons: vec![],
            entities: vec![],
            item: "blueprint",
            label: label.as_ref().to_string(),
            description: description.as_ref().to_string(),
        };
    }

    pub fn add_icon(&mut self, item: &'static str) {
        let val = Icon {
            signal: Signal {
                type_: "item",
                name: item,
            },
            index: self.icons.len() as u8 + 1,
        };
        self.icons.push(val);
    }

    pub fn add(&mut self, item: &'static str, x: f32, y: f32) {
        let num = self.entities.len() as u32 + 1;
        let next = Entity {
            entity_number: num,
            name: item,
            position: Position { x: x, y: y },
        };
        self.entities.push(next);
    }

    pub fn render(&self, w: &mut dyn std::io::Write) -> Result<(), Box<dyn Error>> {
        w.write(b"0")?;
        let bw = EncoderWriter::new(w, base64::STANDARD);
        let ze = ZlibEncoder::new(bw, Compression::best());
        let wrapper: HashMap<_, _> = Some(("blueprint", &self)).into_iter().collect();
        serde_json::to_writer(ze, &wrapper)?;
        Ok(())
    }
}
