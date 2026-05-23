use crate::{
    mission::{Data, IceInfo, IceInstanceInfo, SystemInfo},
    util::Point,
};

pub fn simple_data() -> Data {
    Data {
        ice: vec![IceInfo {
            name: String::default(),
            sprite: String::default(),
        }],
        systems: vec![simple_test_system()],
    }
}

pub fn simple_test_system() -> SystemInfo {
    SystemInfo {
        name: "first".to_string(),
        ice: vec![
            IceInstanceInfo {
                name: String::default(),
                id: 0,
                position: Point::new(0, 0),
                outputs: vec![1, 2],
            },
            IceInstanceInfo {
                name: String::default(),
                id: 1,
                position: Point::new(0, 0),
                outputs: vec![],
            },
            IceInstanceInfo {
                name: String::default(),
                id: 2,
                position: Point::new(0, 0),
                outputs: vec![0],
            },
            IceInstanceInfo {
                name: String::default(),
                id: 3,
                position: Point::new(0, 0),
                outputs: vec![],
            },
        ],
    }
}
