use std::{net::Ipv4Addr, time::Duration};

fn main() {
    println!("Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_8888() {
        ping::ping(
            std::net::IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            Some(Duration::from_secs(2)),
            None,
            None,
            None,
            None,
        )
        .expect("failed to ping");
    }
}
