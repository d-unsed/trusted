use ruru::{Object, RString, Symbol};

use ruby::Config as RubyConfig;

use super::BindingType;

pub struct Config {
    binding_type: BindingType,
    listen_on: String,
}

impl Config {
    pub fn new(binding_type: BindingType, listen_on: String) -> Self {
        Config {
            binding_type: binding_type,
            listen_on: listen_on,
        }
    }

    #[inline]
    pub fn binding_type(&self) -> &BindingType {
        &self.binding_type
    }

    #[inline]
    pub fn listen_on(&self) -> &str {
        &self.listen_on
    }
}


impl From<RubyConfig> for Config {
    fn from(ruby_config: RubyConfig) -> Self {
        // TODO: raise an exception if it is not a String
        let listen_on =
            ruby_config
                .send("listen_on", vec![])
                .try_convert_to::<RString>().unwrap()
                .to_string();

        // TODO: raise an exception if it is not a Symbol
        let binding_type =
            ruby_config
                .send("binding_type", vec![])
                .try_convert_to::<Symbol>().unwrap()
                .to_string();

        let binding_type = BindingType::from(binding_type.as_str());

        Config::new(binding_type, listen_on)
    }
}
