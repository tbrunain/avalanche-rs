#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;

pub mod avm;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
