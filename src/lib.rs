#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod foo {}

// use serde::{Deserialize, Serialize};
pub mod store {
    pub mod db;
    pub mod s3;
}

pub mod actions;
pub mod auth;
