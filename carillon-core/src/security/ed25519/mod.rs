use ed25519_dalek::{Signer, Verifier};
use rand::{CryptoRng, RngCore};

use crate::error::Detail;
use crate::Result;
use crate::security;

const SIGNATURE_SIZE: usize = 64;

pub struct Ed25519 {}

impl security::Algorithm for Ed25519 {
  fn id() -> &'static str { "ed25519" }
}

impl security::PublicKeyAlgorithm for Ed25519 {
  fn generate_key_pair_from<CSPRNG>(csprng: &mut CSPRNG) -> Box<dyn security::KeyPair>
    where CSPRNG: CryptoRng + RngCore {
    let key_pair = ed25519_dalek::Keypair::generate(csprng);
    Box::new(Ed25519KeyPair { key_pair })
  }
}

pub struct Ed25519KeyPair {
  key_pair: ed25519_dalek::Keypair
}

impl security::KeyPair for Ed25519KeyPair {
  fn get_public_key(&self) -> Box<dyn security::PublicKey> {
    Box::new(Ed25519PublicKey { public_key: self.key_pair.public })
  }

  fn generate_signature(&self, message: &[u8]) -> Result<security::Signature> {
    match self.key_pair.try_sign(message) {
      Ok(signature) => {
        Ok(signature.to_bytes().to_vec())
      }
      Err(err) =>
        Err(crate::error::Detail::FailedToSign { message: err.to_string() }),
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ed25519PublicKey {
  public_key: ed25519_dalek::PublicKey
}

impl security::PublicKey for Ed25519PublicKey {
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
  fn to_bytes(&self) -> Vec<u8> {
    self.public_key.to_bytes().to_vec()
  }
  fn from_bytes(bytes: &[u8]) -> Result<Self> where Self: Sized {
    match ed25519_dalek::PublicKey::from_bytes(bytes) {
      Ok(public_key) => Ok(Ed25519PublicKey { public_key }),
      Err(err) => Err(Detail::IncompatibleKeyConversions { message: err.to_string() })
    }
  }
}
