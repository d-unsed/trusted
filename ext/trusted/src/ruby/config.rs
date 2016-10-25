use ruru::{Object, VerifiedObject};

class!(Config);

impl VerifiedObject for Config {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.respond_to("binding_type") && object.respond_to("listen_on")
    }

    fn error_message() -> &'static str {
        "Error converting to Config"
    }
}
