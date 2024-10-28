use std::{ fs, io::Error };
use serde_json::Value;
use crate::utils::config_reader;

pub struct Instance {
    pub path: String,
    pub config: Value,
}

pub fn read(name: &str) -> Result<Vec<Instance>, Error> {
    let mut instances: Vec<Instance> = vec![];
    let dirs = fs::read_dir(name)?;

    for dir in dirs {
        let instance_path = if
            let Ok(mut path) = dir?.file_name().into_string()
        {
            path.insert_str(0, "instances/");
            path
        } else {
            continue;
        };

        let mut instance_config = instance_path.clone();
        instance_config.push_str("/config.mccl.json");

        if let Ok(config) = config_reader::json(&instance_config) {
            instances.push(Instance { path: instance_path, config });
        }
    }

    Ok(instances)
}
