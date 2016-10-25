use super::BindingType;

pub struct Config {
    binding_type: BindingType,
    listen_on: String,
}

impl Config {
    fn new(binding_type: BindingType, listen_on: String) -> Self {
        Config {
            binding_type: binding_type,
            listen_on: listen_on,
        }
    }
}
