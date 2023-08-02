use angry_purple_tiger::AnimalName;
use helium_crypto::{ed25519, Network};
use std::io::{Error as IoError, Write};

pub struct Key {
    pub keypair: ed25519::Keypair,
    pub address: String,
    pub name: String,
}

impl Key {
    pub fn generate(network: Network) -> Self {
        use rand::rngs::OsRng;
        let keypair = ed25519::Keypair::generate(network, &mut OsRng);
        let address = keypair.public_key.to_string();
        let name = address
            .parse::<AnimalName>()
            .expect("failed to parse animal name")
            .to_string();
        Key {
            keypair,
            address,
            name,
        }
    }

    pub fn write(&self, writer: &mut dyn Write) -> Result<(), IoError> {
        writer.write_all(&self.keypair.to_vec())?;
        writer.write_all(&self.keypair.public_key.to_vec())?;
        Ok(())
    }
}
