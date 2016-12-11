use ruru::{Fixnum, Object, RString, Symbol};

use ruby::Config as RubyConfig;

use super::BindingType;

pub struct Config {
    binding_type: BindingType,
    listen_on: String,
    native_thread_pool_size: usize,
    rack_thread_pool_size: usize,
}

impl Config {
    pub fn new(binding_type: BindingType,
               listen_on: String,
               native_thread_pool_size: usize,
               rack_thread_pool_size: usize) -> Self {

        Config {
            binding_type: binding_type,
            listen_on: listen_on,
            native_thread_pool_size: native_thread_pool_size,
            rack_thread_pool_size: rack_thread_pool_size,
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

    #[inline]
    pub fn native_thread_pool_size(&self) -> usize {
        self.native_thread_pool_size
    }

    #[inline]
    pub fn rack_thread_pool_size(&self) -> usize {
        self.rack_thread_pool_size
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

        // TODO: raise an exception if it is not a Symbol
        let native_thread_pool_size =
            ruby_config
                .send("native_thread_pool_size", vec![])
                .try_convert_to::<Fixnum>().unwrap()
                .to_i64() as usize;

        // TODO: raise an exception if it is not a Symbol
        let rack_thread_pool_size =
            ruby_config
                .send("rack_thread_pool_size", vec![])
                .try_convert_to::<Fixnum>().unwrap()
                .to_i64() as usize;

        Config::new(binding_type, listen_on, native_thread_pool_size, rack_thread_pool_size)
    }
}
