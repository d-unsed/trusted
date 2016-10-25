use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum BindingType {
    Tcp,
    Unix,
}

impl<'a> From<&'a str> for BindingType {
    fn from(string: &str) -> Self {
        match string {
            "tcp" => BindingType::Tcp,
            "unix" => BindingType::Unix,

            // TODO: move to `Result` and return `Err`
            _ => panic!("Unsupported binding type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_converted_from_tcp() {
        assert_eq!(BindingType::from("tcp"), BindingType::Tcp);
    }

    #[test]
    fn it_is_converted_from_unix() {
        assert_eq!(BindingType::from("unix"), BindingType::Unix);
    }
}
