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
struct Signal {
    #[serde(rename = "type")] // <-- this is also a container attribute
    type_: &'static str,
    name: String,
}

#[derive(Debug, Serialize)]
struct Icon {
    signal: Signal,
    index: u8,
}

#[derive(Debug, Serialize)]
struct Filter {
    signal: Signal,
    count: i32,
    index: u8,
}

#[derive(Debug, Serialize)]
struct ControlBehavior {
    filters: Vec<Filter>,
}

#[derive(Debug, Serialize)]
struct InventoryFilter {
    index: usize,
    name: String,
}

#[derive(Debug, Serialize)]
struct InventoryFilters {
    filters: Vec<InventoryFilter>,
}

#[derive(Debug, Serialize)]
struct Entity {
    entity_number: usize,
    name: &'static str,
    position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_behavior: Option<ControlBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<HashMap<String, usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inventory: Option<InventoryFilters>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Comparison {
    AND,
    OR,
}

#[derive(Debug, Serialize)]
pub struct ScheduleCondition {
    compare_type: Comparison,
    #[serde(rename = "type")]
    type_: String,
}

impl ScheduleCondition {
    pub fn and(t: String) -> Self {
        return ScheduleCondition {
            compare_type: Comparison::AND,
            type_: t,
        };
    }
    pub fn or(t: String) -> Self {
        return ScheduleCondition {
            compare_type: Comparison::OR,
            type_: t,
        };
    }
}

#[derive(Debug, Serialize)]
pub struct Stop {
    station: String,
    wait_conditions: Vec<ScheduleCondition>,
}

impl Stop {
    pub fn new(station: String, wait_conditions: Vec<ScheduleCondition>) -> Self {
        return Stop {
            station: station,
            wait_conditions: wait_conditions,
        };
    }
}

#[derive(Debug, Serialize)]
struct Schedule {
    locomotives: Vec<usize>,
    schedule: Vec<Stop>,
}

pub trait Render {
    fn render(&self, w: &mut dyn std::io::Write) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Serialize)]
pub struct BlueprintBuilder {
    icons: Vec<Icon>,
    entities: Vec<Entity>,
    schedules: Vec<Schedule>,
    item: &'static str,
    label: String,
    description: String,
}

impl BlueprintBuilder {
    pub fn new<S1, S2>(label: S1, description: S2) -> BlueprintBuilder
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        return BlueprintBuilder {
            icons: vec![],
            entities: vec![],
            schedules: vec![],
            item: "blueprint",
            label: label.as_ref().to_string(),
            description: description.as_ref().to_string(),
        };
    }

    pub fn add_icon<S: AsRef<str>>(&mut self, item: S) {
        let val = Icon {
            signal: Signal {
                type_: "item",
                name: item.as_ref().to_string(),
            },
            index: self.icons.len() as u8 + 1,
        };
        self.icons.push(val);
    }

    pub fn add(&mut self, item: &'static str, x: f32, y: f32) -> usize {
        return self.add_impl(item, x, y, None, None, None);
    }

    pub fn add_with_items(
        &mut self,
        item: &'static str,
        x: f32,
        y: f32,
        items: Vec<(&'static str, usize)>,
    ) -> usize {
        return self.add_impl(
            item,
            x,
            y,
            Some(items.iter().map(|(x, y)| (x.to_string(), *y)).collect()),
            None,
            None,
        );
    }

    pub fn add_with_inventory(
        &mut self,
        item: &'static str,
        x: f32,
        y: f32,
        storage: &String,
    ) -> usize {
        let filters = Some(InventoryFilters {
            filters: (1_usize..=40_usize)
                .map(|x| InventoryFilter {
                    name: storage.to_string(),
                    index: x,
                })
                .collect(),
        });
        return self.add_impl(item, x, y, None, None, filters);
    }

    pub fn add_combinator(&mut self, x: f32, y: f32, signals: Vec<(&'static str, i32)>) -> usize {
        let filters = signals
            .iter()
            .enumerate()
            .map(|(i, (x, y))| Filter {
                index: i as u8 + 1,
                signal: Signal {
                    type_: "item",
                    name: x.to_string(),
                },
                count: *y,
            })
            .collect();
        return self.add_impl(
            "constant-combinator",
            x,
            y,
            None,
            Some(ControlBehavior { filters: filters }),
            None,
        );
    }
    fn add_impl(
        &mut self,
        item: &'static str,
        x: f32,
        y: f32,
        items: Option<HashMap<String, usize>>,
        control_behavior: Option<ControlBehavior>,
        inventory: Option<InventoryFilters>,
    ) -> usize {
        let num = self.entities.len() + 1;
        let next = Entity {
            entity_number: num,
            name: item,
            position: Position { x: x, y: y },
            control_behavior: control_behavior,
            items: items,
            inventory: inventory,
        };
        self.entities.push(next);
        return num;
    }

    pub fn add_schedule(&mut self, trains: Vec<usize>, stops: Vec<Stop>) {
        self.schedules.push(Schedule {
            locomotives: trains,
            schedule: stops,
        })
    }
}

impl Render for BlueprintBuilder {
    fn render(&self, w: &mut dyn std::io::Write) -> Result<(), Box<dyn Error>> {
        w.write(b"0")?;
        let bw = EncoderWriter::new(w, base64::STANDARD);
        let ze = ZlibEncoder::new(bw, Compression::best());
        let wrapper: HashMap<_, _> = Some(("blueprint", &self)).into_iter().collect();
        serde_json::to_writer(ze, &wrapper)?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct BookBuilder {
    icons: Vec<Icon>,
    blueprints: Vec<HashMap<&'static str, BlueprintBuilder>>,
    item: &'static str,
    label: String,
    description: String,
}

impl BookBuilder {
    pub fn new<S1, S2>(label: S1, description: S2) -> BookBuilder
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        return BookBuilder {
            icons: vec![],
            blueprints: vec![],
            item: "blueprint-book",
            label: label.as_ref().to_string(),
            description: description.as_ref().to_string(),
        };
    }

    pub fn add_icon(&mut self, item: &'static str) {
        let val = Icon {
            signal: Signal {
                type_: "item",
                name: item.to_string(),
            },
            index: self.icons.len() as u8 + 1,
        };
        self.icons.push(val);
    }
    pub fn add_blueprint(&mut self, bp: BlueprintBuilder) {
        self.blueprints
            .push(Some(("blueprint", bp)).into_iter().collect());
    }
}

impl Render for BookBuilder {
    fn render(&self, w: &mut dyn std::io::Write) -> Result<(), Box<dyn Error>> {
        w.write(b"0")?;
        let bw = EncoderWriter::new(w, base64::STANDARD);
        let ze = ZlibEncoder::new(bw, Compression::best());
        let wrapper: HashMap<_, _> = Some(("blueprint_book", &self)).into_iter().collect();
        serde_json::to_writer(ze, &wrapper)?;
        Ok(())
    }
}
