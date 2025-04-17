use audio_device::AudioDevice;
use audio_source::AudioSource;
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use std::str::FromStr;

use types::{IpAddress, MacAddress};

mod audio_device;
mod audio_source;
mod err;
mod types;
mod utils;

lazy_static! {
    static ref SERVER_IP_ADDRESS: types::IpAddress = {
        let ip = local_ip().unwrap().to_string();
        types::IpAddress::from_str(&ip).unwrap()
    };
}

fn main() {
    println!("Hello, {}!", *SERVER_IP_ADDRESS);
    let lappy = AudioSource::new(
        "Lappy".to_string(),
        "spandan".to_string(),
        IpAddress::from_str("192.168.1.40").unwrap(),
    );
    let homesrv = AudioSource::new(
        "Homesrv".to_string(),
        "spandan".to_string(),
        IpAddress::from_str("192.168.1.37").unwrap(),
    );

    let speaker = AudioDevice::new(
        "Speaker".to_string(),
        MacAddress::from_str("F4:4E:FD:0A:DC:4C").unwrap(),
    );

    lappy.disconnect(&speaker);
    homesrv.connect(&speaker);
    std::thread::sleep(std::time::Duration::from_secs(20));
    homesrv.disconnect(&speaker);
    lappy.connect(&speaker);
}
