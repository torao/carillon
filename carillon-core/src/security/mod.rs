use num::BigUint;
use rand::{CryptoRng, RngCore};

use crate::Result;

pub mod ed25519;

pub fn rng<T>(r: T) -> T where T: CryptoRng + RngCore {
  r
}

/// 公開鍵アルゴリズムの機能を表す構造体。
///
pub struct PublicKeyImpl {
  id: &'static str,
  generate_key_pair: fn() -> Box<dyn KeyPair + 'static>,
  generate_key_pair_from: fn(u64) -> Box<dyn KeyPair + 'static>,
  restore_key_pair: fn(&[u8]) -> Result<Box<dyn KeyPair + 'static>>,
}

impl Default for PublicKeyImpl {
  fn default() -> Self {
    ed25519::algorithm()
  }
}

impl PublicKeyImpl {
  /// このアルゴリズムの ID を参照します。アルゴリズムの ID 文字列は URI や JSON のキー、シリアライズされたデータの
  /// スキームとして使用される可能性があります。
  pub fn id(&self) -> &'static str {
    self.id
  }

  /// この公開鍵アルゴリズムを使用して暗号論的ランダムさが保証されている鍵ペア (公開鍵/秘密鍵のセット) を生成します。
  ///
  pub fn generate_key_pair(&self) -> Box<dyn KeyPair> {
    (self.generate_key_pair)()
  }

  /// この公開鍵アルゴリズムに基づいて指定されたシードに関連する公開鍵ペアを決定論的に生成します。
  ///
  pub fn generate_key_pair_from(&self, seed: u64) -> Box<dyn KeyPair + 'static> {
    (self.generate_key_pair_from)(seed)
  }

  /// 指定されたバイト列からこのアルゴリズムに対応する鍵ペアを復元します。
  pub fn restore_key_pair(&self, bytes: &[u8]) -> Result<Box<dyn KeyPair + 'static>> {
    (self.restore_key_pair)(bytes)
  }
}

/// 公開鍵ペアを表すトレイト。
///
pub trait KeyPair {
  /// この鍵ペアの秘密鍵をバイト列に変換します。
  fn to_bytes(&self) -> Vec<u8>;

  /// 指定されたバイト列から鍵ペアを復元します。
  // fn from_bytes(bytes: &[u8]) -> Result<Self>
  //   where
  //     Self: Sized;

  /// この鍵ペアの公開鍵を参照します。
  fn public_key(&self) -> Box<dyn PublicKey>;

  /// 秘密鍵を使用してメッセージに対する署名を生成します。
  /// 生成された署名はペアとなる公開鍵の `verify_signature()` で検証することができます。
  fn generate_signature(&self, message: &[u8]) -> Result<Signature>;
}

/// 公開鍵を表すトレイト。
pub trait PublicKey {
  /// この公開鍵をバイト列に変換します。返値のバイト列を `from_bytes()` に適用することで公開鍵を復元することが
  ///できます。
  fn to_bytes(&self) -> Vec<u8>;

  /// この公開鍵のアドレスを参照します。
  fn address(&self) -> String {
    BigUint::from_bytes_be(self.to_bytes().as_slice()).to_str_radix(36)
  }

  /// 指定されたバイト列から公開鍵を復元します。
  fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
      Self: Sized;

  /// この公開鍵に対する秘密鍵によって作成された署名を検証します。
  fn verify_signature(&self, signature: Signature, message: &[u8]) -> Result<bool>;
}

/// 署名を表すバイト列。
pub type Signature = Vec<u8>;
