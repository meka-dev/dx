use anyhow::{anyhow, Error};

#[derive(Debug, PartialEq)]
pub struct Event(String);

impl Event {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TryFrom<String> for Event {
    type Error = Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.len() % 2 != 0 {
            return Err(anyhow!(
                "incorrect length for hex encoding: {}",
                input.len()
            ));
        }

        if !input.bytes().all(is_hex) {
            return Err(anyhow!("found invalid hex character"));
        }

        Ok(Self(input))
    }
}

fn is_hex(byte: u8) -> bool {
    (byte > 0x2F && byte < 0x3A) || (byte > 0x40 && byte < 0x47) || (byte > 0x60 && byte < 0x67)
}
