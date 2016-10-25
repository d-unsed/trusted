use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum BindingType {
    Tcp,
    Unix,
}
