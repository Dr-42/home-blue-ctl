use crate::{SERVER_IP_ADDRESS, audio_device::AudioDevice, types::IpAddress};

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
        println!("Connecting {} to {}", self.name(), device.name());
        self.run_command(&format!("bluetoothctl connect {}", device.mac_address()));
        println!("Connected {} to {}", self.name, device.name());
    }

    pub fn disconnect(&self, device: &AudioDevice) {
        println!("Disconnecting {} from {}", self.name, device.name());
        self.run_command(&format!("bluetoothctl disconnect {}", device.mac_address()));
        println!("Disconnected {} from {}", self.name, device.name());
    }
}
