use crate::types::MacAddress;

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
