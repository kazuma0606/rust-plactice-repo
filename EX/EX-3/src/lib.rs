#![allow(non_snake_case)] 

pub mod core {
    pub mod config {
        pub mod settings;
    }
    pub mod dto {
        pub mod user;
    }
    pub mod traits {
        pub mod create_user;
        pub mod find_all_user;
    }
    pub mod util {
        pub mod check_empty;
    }
}