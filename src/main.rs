use lazy_static::lazy_static;
use local_ip_address::local_ip;
use std::{fmt::Display, str::FromStr};

lazy_static! {
    static ref SERVER_IP_ADDRESS: IpAddress = {
        let ip = local_ip().unwrap().to_string();
        IpAddress::from_str(&ip).unwrap()
    };
}

#[derive(Debug)]
pub enum Error {
    MacAddressParseError,
}

#[derive(Copy, Clone)]
pub struct MacAddress {
    bytes: [u8; 6],
}

impl FromStr for MacAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .split(':')
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Ok(MacAddress { bytes })
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5]
        )
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct IpAddress {
    bytes: [u8; 4],
}

impl FromStr for IpAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .split('.')
            .map(|b| b.parse().unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Ok(IpAddress { bytes })
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]
        )
    }
}

pub struct AudioDevice {
    name: String,
    mac_address: MacAddress,
}

impl AudioDevice {
    pub fn new(name: String, mac_address: MacAddress) -> Self {
        Self { name, mac_address }
    }

    pub fn mac_address(&self) -> MacAddress {
        self.mac_address
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct AudioSource {
    name: String,
    system_username: String,
    ip_address: IpAddress,
}

impl AudioSource {
    pub fn new(name: String, system_username: String, ip_address: IpAddress) -> Self {
        Self {
            name,
            system_username,
            ip_address,
        }
    }

    pub fn ip_address(&self) -> IpAddress {
        self.ip_address
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn run_command(&self, command: &str) {
        let cmd = if self.ip_address == *SERVER_IP_ADDRESS {
            command.to_string()
        } else {
            let cmd_ssh_prefix = format!("ssh {}@{}", self.system_username, self.ip_address());
            format!("{} -t {}", cmd_ssh_prefix, command)
        };
        println!("Running: {}", cmd);
        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process");

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            eprintln!("Command failed with exit code: {}", output.status);
        } else {
            println!("Command succeeded with exit code: {}", output.status);
        }
    }

    pub fn connect(&self, device: &AudioDevice) {
        println!("Connecting {} to {}", self.name, device.name());
        self.run_command(&format!("bluetoothctl connect {}", device.mac_address()));
        println!("Connected {} to {}", self.name, device.name());
    }

    pub fn disconnect(&self, device: &AudioDevice) {
        println!("Disconnecting {} from {}", self.name, device.name());
        self.run_command(&format!("bluetoothctl disconnect {}", device.mac_address()));
        println!("Disconnected {} from {}", self.name, device.name());
    }
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
