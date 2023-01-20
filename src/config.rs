use std::collections::HashMap;

use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub struct ConfigFile {
    pub file_name: String,
    pub read_at: SystemTime,
    pub last_contents: String,
}

pub struct ConfigChange {
    pub previous_contents: String,
    pub current_contents: String,
}

pub struct ConfigSet {
    pub config_directory: String,
    pub config_maps: HashMap<String, ConfigFile>,
}

impl ConfigSet {
    pub fn new(config_directory: String) -> Self {
        Self {
            config_directory,
            config_maps: HashMap::new(),
        }
    }

    pub fn scan(&mut self) -> Vec<ConfigChange> {
        let mut changes = Vec::new();

        // Scan config directory and add changes into changes vector
        for path in fs::read_dir(&self.config_directory).unwrap() {
            if let Ok(path) = path {
                let file_name = path.file_name().into_string().unwrap();
                let file_contents = fs::read_to_string(path.path()).unwrap();
                let read_at = path.metadata().unwrap().modified().unwrap();

                if let Some(config_file) = self.config_maps.get_mut(&file_name)
                {
                    if config_file.last_contents != file_contents {
                        changes.push(ConfigChange {
                            previous_contents: config_file
                                .last_contents
                                .clone(),
                            current_contents: file_contents.clone(),
                        });
                    }
                    config_file.last_contents = file_contents;
                    config_file.read_at = read_at;
                } else {
                    self.config_maps.insert(
                        file_name.to_string(),
                        ConfigFile {
                            file_name: file_name.to_string(),
                            read_at,
                            last_contents: file_contents.clone(),
                        },
                    );
                    changes.push(ConfigChange {
                        previous_contents: "".to_string(),
                        current_contents: file_contents,
                    });
                }
            }
        }

        // Find deleted files and add changes into changes vector
        for (file_name, config_file) in self.config_maps.iter() {
            if !Path::new(&format!("{}/{}", self.config_directory, file_name))
                .exists()
            {
                changes.push(ConfigChange {
                    previous_contents: config_file.last_contents.clone(),
                    current_contents: "".to_string(),
                });
            }
        }

        // Return changes
        changes
    }
}
