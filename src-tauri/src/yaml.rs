use crate::bd_manager::{BDOperation, SerdePersons};
use std::path::PathBuf;

pub struct YamlBD {
    pub file_path: PathBuf,
}

impl BDOperation for YamlBD {
    fn load(&self) -> Result<crate::bd_manager::SerdePersons, Box<dyn std::error::Error>> {
        let handler = std::fs::File::open(&self.file_path)?;
        let data: SerdePersons = serde_yaml::from_reader(&handler)?;
        println!("{:?}", data);
        Ok(data)
    }
    fn save(
        &self,
        persons: &crate::bd_manager::SerdePersons,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let handler = std::fs::File::create(&self.file_path)?;
        serde_yaml::to_writer(&handler, &persons)?;
        Ok(())
    }
}
