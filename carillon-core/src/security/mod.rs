use rand::{CryptoRng, RngCore};
use rand::rngs::OsRng;

use crate::Result;

pub mod ed25519;

/// セキュリティアルゴリズムの実装を表すトレイト。
///
pub trait Algorithm {
  /// このアルゴリズムの ID を参照します。アルゴリズムの ID 文字列は URI や JSON のキー、シリアライズされたデータの
  /// スキームとして使用される可能性があります。
  ///
  fn id() -> &'static str;
}

/// 公開鍵アルゴリズムを表すトレイト。
///
pub trait PublicKeyAlgorithm: Algorithm {
  /// この公開鍵アルゴリズムを使用して鍵ペア (公開鍵/秘密鍵のセット) を生成します。
  ///
  fn generate_key_pair() -> Box<dyn KeyPair> {
    let mut csprng = OsRng {};
    Self::generate_key_pair_from(&mut csprng)
  }

  /// 指定された CSPRNG (暗号論的擬似乱数生成器) に基づいて新しい公開鍵ペアを生成します。
  ///
  fn generate_key_pair_from<CSPRNG>(csprng: &mut CSPRNG) -> Box<dyn KeyPair>
    where CSPRNG: CryptoRng + RngCore;
}

/// 公開鍵ペアを表すトレイト。
/// この鍵ペア自体が秘密鍵としての機能を実装しています。秘密鍵そのものを参照する手段を提供していない点に注意。
///
pub trait KeyPair {
  /// この鍵ペアの公開鍵を参照します。
  ///
  fn get_public_key(&self) -> Box<dyn PublicKey>;

  /// この鍵ペアの秘密鍵を使用して指定されたメッセージに対する署名を生成します。
  /// 生成された署名は公開鍵の `verify_signature()` を使用して検証することができます。
  ///
  fn generate_signature(&self, message: &[u8]) -> Result<Signature>;
}

/// 公開鍵を表すトレイト。
pub trait PublicKey {
  /// この公開鍵に対する秘密鍵によって作成された署名を検証します。
  fn verify_signature(&self, signature: Signature, message: &[u8]) -> Result<bool>;

  /// この公開鍵をバイト列に変換します。返値のバイト列を `from_bytes()` に適用することで公開鍵を復元することが
  ///できます。
  fn to_bytes(&self) -> Vec<u8>;

  /// 指定されたバイト列から公開鍵を復元します。
  fn from_bytes(bytes: &[u8]) -> Result<Self> where Self: Sized;
}

/// 署名を表すバイト列。
pub type Signature = Vec<u8>;
