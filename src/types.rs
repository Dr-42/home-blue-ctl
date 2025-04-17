use std::{fmt::Display, str::FromStr};

use crate::err::Error;

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
            .try_into();

        match bytes {
            Ok(bytes) => Ok(MacAddress { bytes }),
            Err(_) => Err(Error::MacAddressParseError),
        }
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
