use std::io::{Read, Write};

//
// Some configuration for an app
//
pub struct Config {
    values: Vec<(String, String)>,
}

//
// Handles mamagement of configuraton
//
pub struct KeyValConfigService {}

impl Config {
    pub fn new(values: Vec<(String, String)>) -> Config {
        Config { values }
    }
}

impl KeyValConfigService {
    pub fn new() -> KeyValConfigService {
        KeyValConfigService {}
    }
}

//
// A get() function for returning values for a specific key
//
pub trait ValGetter {
    fn get(&self, s: &str) -> Option<String>;
}

//
// Write to a config
//
pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}

//
// Read a configuration
//
pub trait ConfigReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config>;
}

impl ConfigWriter for KeyValConfigService {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()> {
        for val in config.values {
            writeln!(to, "{0} = {1}", val.0, val.1);
        }

        Ok(())
    }
}

impl ConfigReader for KeyValConfigService {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer);

        let values = buffer
            .split_terminator("\n")
            .map(|l| l.trim())
            .filter(|l| {
                let pos = l.find("=").unwrap_or(0);
                pos > 0 && pos < l.len() - 1
            })
            .map(|l| {
                let parts = l.split("=").collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect();

        Ok(Config { values })
    }
}

impl ValGetter for Config {
    fn get(&self, val: &str) -> Option<String> {
        self.values
            .iter()
            .find_map(|v| if &v.0 == val { Some(v.1.clone()) } else { None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn config_get_value() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);
        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("world"), None);
    }

    #[test]
    fn KeyValConfigService_write_config() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);
        let service = KeyValConfigService::new();
        let mut target = vec![];

        service.write(config, &mut target);
        assert_eq!(
            String::from_utf8(target).unwrap(),
            "hello = world\n".to_string()
        );
    }

    #[test]
    fn KeyValConfigService_read_from_confg() {
        let service = KeyValConfigService::new();

        let readable = &format!("{}\n{}", "hello=world", "a=b").into_bytes();

        let config = service
            .read(&mut Cursor::new(readable))
            .expect("Couldn't read from the vector");

        assert_eq!(
            config.values,
            vec![
                ("hello".to_string(), "world".to_string()),
                ("a".to_string(), "b".to_string())
            ]
        );
    }
}
