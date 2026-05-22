use crate::mission::*;
use crate::prelude::*;

const ICE_JSON: &str = include_str!("../../data/ice.json");
const SYSTEMS_JSON: &str = include_str!("../../data/systems.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceInfo {
    pub name: String,
    pub sprite: String,
}

impl IceInfo {
    pub fn instance(&self, id: u32, position: Point) -> Ice {
        Ice {
            name: self.name.clone(),
            sprite: self.sprite.clone(),
            position,
            id: IceId(id),
            inputs: vec![],
            outputs: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceInstanceInfo {
    pub name: String,
    pub id: u32,
    pub position: Point,
    pub inputs: Vec<u32>,
    pub outputs: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    pub ice: Vec<IceInstanceInfo>,
}

impl SystemInfo {
    pub fn instance(&self, data: &Data) -> System {
        let ice = self
            .ice
            .iter()
            .map(|ice_info| {
                let mut ice = data
                    .get_ice_info(&ice_info.name)
                    .instance(ice_info.id, ice_info.position);
                ice.inputs = ice_info.inputs.iter().map(|i| IceId(*i)).collect();
                ice.outputs = ice_info.outputs.iter().map(|i| IceId(*i)).collect();
                ice
            })
            .collect();
        System {
            name: self.name.clone(),
            ice,
        }
    }
}

pub struct Data {
    ice: Vec<IceInfo>,
    systems: Vec<SystemInfo>,
}

impl Data {
    pub fn load() -> Result<Self, serde_json::Error> {
        let ice = serde_json::from_str(ICE_JSON)?;
        let systems = serde_json::from_str(SYSTEMS_JSON)?;
        Ok(Self { ice, systems })
    }

    pub fn get_ice_info(&self, name: &str) -> &IceInfo {
        self.ice
            .iter()
            .find(|e| e.name == name)
            .expect(&format!("Unable to load ice data for: {}", name))
    }

    pub fn get_system_info(&self, name: &str) -> &SystemInfo {
        self.systems
            .iter()
            .find(|e| e.name == name)
            .expect(&format!("Unable to load system data for: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use crate::mission::*;

    #[test]
    fn can_parse() {
        let data = Data::load().unwrap();
        let _ = data.get_system_info("Intro").instance(&data);
    }
}
