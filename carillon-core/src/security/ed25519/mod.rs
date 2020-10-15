use ed25519_dalek::{Signer, Verifier};
use rand::prelude::StdRng;
use rand::rngs::OsRng;
use rand::SeedableRng;
use rand::{CryptoRng, RngCore};

use crate::error::Detail;
use crate::security;
use crate::security::PublicKeyImpl;
use crate::Result;

const SIGNATURE_SIZE: usize = 64;

pub fn algorithm() -> PublicKeyImpl {
  PublicKeyImpl { id: "ed25519", generate_key_pair, generate_key_pair_from, restore_key_pair }
}

fn generate_key_pair() -> Box<dyn security::KeyPair> {
  let mut csprng = OsRng {};
  generate_key_pair_from_csprng(&mut csprng)
}

fn generate_key_pair_from(seed: u64) -> Box<dyn security::KeyPair> {
  let mut csprng = StdRng::seed_from_u64(seed);
  generate_key_pair_from_csprng(&mut csprng)
}

fn generate_key_pair_from_csprng<CSPRNG>(csprng: &mut CSPRNG) -> Box<dyn security::KeyPair>
where
  CSPRNG: CryptoRng + RngCore,
{
  let key_pair = ed25519_dalek::Keypair::generate(csprng);
  Box::new(KeyPair { key_pair })
}

fn restore_key_pair(bytes: &[u8]) -> Result<Box<dyn security::KeyPair>> {
  match ed25519_dalek::Keypair::from_bytes(bytes) {
    Ok(key_pair) => Ok(Box::new(KeyPair { key_pair })),
    Err(err) => Err(Detail::CannotRestoreKey { message: err.to_string() }),
  }
}

pub struct KeyPair {
  key_pair: ed25519_dalek::Keypair,
}

impl security::KeyPair for KeyPair {
  fn to_bytes(&self) -> Vec<u8> {
    self.key_pair.to_bytes().to_vec()
  }

  fn public_key(&self) -> Box<dyn security::PublicKey> {
    Box::new(PublicKey { public_key: self.key_pair.public })
  }

  fn generate_signature(&self, message: &[u8]) -> Result<security::Signature> {
    match self.key_pair.try_sign(message) {
      Ok(signature) => Ok(signature.to_bytes().to_vec()),
      Err(err) => Err(crate::error::Detail::FailedToSign { message: err.to_string() }),
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublicKey {
  public_key: ed25519_dalek::PublicKey,
}

impl security::PublicKey for PublicKey {
  fn to_bytes(&self) -> Vec<u8> {
    self.public_key.to_bytes().to_vec()
  }
  fn from_bytes(bytes: &[u8]) -> Result<Self>
  where
    Self: Sized,
  {
    match ed25519_dalek::PublicKey::from_bytes(bytes) {
      Ok(public_key) => Ok(PublicKey { public_key }),
      Err(err) => Err(Detail::IncompatibleKeyConversions { message: err.to_string() }),
    }
  }
  fn verify_signature(&self, signature: security::Signature, message: &[u8]) -> Result<bool> {
    if signature.len() == SIGNATURE_SIZE {
      let mut ed25519_signature = [0u8; SIGNATURE_SIZE];
      // TODO use unsafe memcpy()
      for i in 0..signature.len() {
        ed25519_signature[i] = signature[i];
      }
      let sigs = ed25519_dalek::Signature::new(ed25519_signature);
      Ok(self.public_key.verify(message, &sigs).is_ok())
    } else {
      let message = String::from("not an ed25519 signature");
      Err(Detail::IncompatibleKeyConversions { message })
    }
  }
}
