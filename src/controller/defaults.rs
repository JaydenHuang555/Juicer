use crate::controller::Controller;

use once_cell::sync::OnceCell;
use std::net::{IpAddr, Ipv4Addr};

use fetchlib::key::credentials::Credentials;

pub static RIO: OnceCell<Controller> = OnceCell::new();

pub static SYSTEMCORE: OnceCell<Controller> = OnceCell::new();

pub fn rio() -> &'static Controller {
    RIO.get_or_init(|| Controller {
        default_addr: IpAddr::V4(Ipv4Addr::new(172, 22, 11, 2)),
        hostid: 2,
        credentials: Credentials {
            username: "lvuser".to_string(),
            password: None,
        },
    })
}

pub fn systemcore() -> &'static Controller {
    SYSTEMCORE.get_or_init(|| Controller {
        default_addr: IpAddr::V4(Ipv4Addr::new(172, 30, 0, 1)),
        hostid: 2,
        credentials: Credentials {
            username: "SYSTEMCORE".to_string(),
            password: Some("PASSWORD".to_string()),
        },
    })
}
