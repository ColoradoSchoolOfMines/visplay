use openssl::sign::Verifier;
use openssl::pkey::PKey;
use openssl::rand::rand_bytes;
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;

use ::Config;

use std::fs::File;
use std::io::Read;

pub struct Auth {
    pub keys: Vec<PKey>,
}

impl Auth {
    pub fn from_cfg(cfg: &Config) -> Auth {
        let mut keys = Vec::new();

        for path in &cfg.keys {
            if path.exists() && path.is_file() {
                debug!("Searching for key in {}", path.display());
                
                let mut f = File::open(path).expect("Error opening key file!");
                let mut bytes = Vec::new();
                f.read_to_end(&mut bytes).expect("Could not read key file!");
                keys.push(match PKey::public_key_from_pem(&bytes) {
                    Ok(k) => k,
                    Err(e) => { 
                        error!("Invalid public key file \"{}\": {}", path.display(), e);
                        continue;
                    },
                });

            } else {
                error!("PEM key file \"{}\" does not exist, skipping", path.display());
            }
        }

        Auth {
            keys: keys,
        }
    }

    pub fn gen_check<'a>(&'a self, bytes: usize) -> Result<Check<'a>, ErrorStack> {
        let mut bytes = vec![0u8; bytes];
        rand_bytes(&mut bytes)?;

        Ok(Check {
            auth: self,
            bytes: bytes,
            digest: MessageDigest::sha256(),
        })
    }
}

pub struct Check<'a> {
    auth: &'a Auth,
    pub bytes: Vec<u8>,
    pub digest: MessageDigest,
}

impl<'a> Check<'a> {
    pub fn verif(&self, sig: &[u8]) -> Result<bool, ErrorStack> {
        for key in &self.auth.keys {
            let mut verif = Verifier::new(self.digest, key)?;
            verif.update(&self.bytes)?;
            if verif.finish(sig)? { 
                trace!("Key accepted");
                return Ok(true) 
            }
        }

        Ok(false)
    }
}