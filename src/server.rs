pub mod space_command {
    pub mod net {
        include!(concat!(env!("OUT_DIR"), "/space_command.net.rs"));
    }
}

use space_command::net;

