//! Cipher Kind

#[cfg(feature = "v1-aead")]
use super::aeadcipher::{Aes128Gcm, Aes256Gcm, ChaCha20Poly1305};
#[cfg(feature = "v1-aead-extra")]
use super::aeadcipher::{Aes128GcmSiv, Aes256GcmSiv, XChaCha20Poly1305};

#[cfg(feature = "v1-stream")]
use super::streamcipher::{
    Aes128Cfb1,
    Aes128Cfb128,
    Aes128Cfb8,
    Aes128Ctr,
    Aes128Ofb,
    Aes192Cfb1,
    Aes192Cfb128,
    Aes192Cfb8,
    Aes192Ctr,
    Aes192Ofb,
    Aes256Cfb1,
    Aes256Cfb128,

    Aes256Cfb8,
    Aes256Ctr,
    Aes256Ofb,
    Camellia128Cfb1,
    Camellia128Cfb128,
    Camellia128Cfb8,
    Camellia128Ctr,
    Camellia128Ofb,
    Camellia192Cfb1,
    Camellia192Cfb128,
    Camellia192Cfb8,
    Camellia192Ctr,
    Camellia192Ofb,
    Camellia256Cfb1,
    Camellia256Cfb128,

    Camellia256Cfb8,
    Camellia256Ctr,
    Camellia256Ofb,

    Chacha20,
    Rc4,
    Rc4Md5,
};

/// Category of ciphers
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CipherCategory {
    /// No encryption
    None,
    /// Stream ciphers is used for OLD ShadowSocks protocol, which uses stream ciphers to encrypt data payloads
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    Stream,
    /// AEAD ciphers is used in modern ShadowSocks protocol, which sends data in separate packets
    #[cfg(feature = "v1-aead")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead")))]
    Aead,
}

/// ShadowSocks cipher type
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CipherKind {
    NONE,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    SS_TABLE,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    SS_RC4_MD5,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_128_CTR,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_192_CTR,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_256_CTR,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_128_CFB1,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_128_CFB8,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_128_CFB128,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_192_CFB1,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_192_CFB8,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_192_CFB128,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_256_CFB1,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_256_CFB8,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_256_CFB128,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_128_OFB,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_192_OFB,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    AES_256_OFB,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_128_CTR,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_192_CTR,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_256_CTR,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_128_CFB1,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_128_CFB8,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_128_CFB128,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_192_CFB1,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_192_CFB8,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_192_CFB128,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_256_CFB1,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_256_CFB8,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_256_CFB128,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_128_OFB,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_192_OFB,
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CAMELLIA_256_OFB,

    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    RC4,
    // NOTE: IETF 版本
    #[cfg(feature = "v1-stream")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-stream")))]
    CHACHA20,

    // AEAD Cipher
    #[cfg(feature = "v1-aead")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead")))]
    /// AEAD_AES_128_GCM
    AES_128_GCM,
    #[cfg(feature = "v1-aead")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead")))]
    /// AEAD_AES_256_GCM
    AES_256_GCM,

    #[cfg(feature = "v1-aead-extra")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead-extra")))]
    /// AEAD_AES_128_GCM_SIV
    AES_128_GCM_SIV,
    #[cfg(feature = "v1-aead-extra")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead-extra")))]
    /// AEAD_AES_256_GCM_SIV
    AES_256_GCM_SIV,

    // NOTE: IETF 版本
    #[cfg(feature = "v1-aead")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead")))]
    /// AEAD_CHACHA20_POLY1305
    CHACHA20_POLY1305,

    #[cfg(feature = "v1-aead-extra")]
    #[cfg_attr(docrs, doc(cfg(feature = "v1-aead-extra")))]
    /// AEAD_XCHACHA20_POLY1305
    XCHACHA20_POLY1305,
}

impl CipherKind {
    /// The category of the cipher
    pub fn category(&self) -> CipherCategory {
        #[cfg(feature = "v1-stream")]
        if self.is_stream() {
            return CipherCategory::Stream;
        }

        #[cfg(feature = "v1-aead")]
        if self.is_aead() {
            return CipherCategory::Aead;
        }

        CipherCategory::None
    }

    /// Check if the current cipher is `NONE`
    pub fn is_none(&self) -> bool {
        matches!(*self, CipherKind::NONE)
    }

    /// Check if the current cipher is a stream cipher
    #[cfg(feature = "v1-stream")]
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_stream(&self) -> bool {
        use self::CipherKind::*;

        match *self {
            SS_TABLE | SS_RC4_MD5 | AES_128_CTR | AES_192_CTR | AES_256_CTR | AES_128_CFB1 | AES_128_CFB8
            | AES_128_CFB128 | AES_192_CFB1 | AES_192_CFB8 | AES_192_CFB128 | AES_256_CFB1 | AES_256_CFB8
            | AES_256_CFB128 | AES_128_OFB | AES_192_OFB | AES_256_OFB | CAMELLIA_128_CTR | CAMELLIA_192_CTR
            | CAMELLIA_256_CTR | CAMELLIA_128_CFB1 | CAMELLIA_128_CFB8 | CAMELLIA_128_CFB128 | CAMELLIA_192_CFB1
            | CAMELLIA_192_CFB8 | CAMELLIA_192_CFB128 | CAMELLIA_256_CFB1 | CAMELLIA_256_CFB8 | CAMELLIA_256_CFB128
            | CAMELLIA_128_OFB | CAMELLIA_192_OFB | CAMELLIA_256_OFB | RC4 | CHACHA20 => true,
            _ => false,
        }
    }

    /// Check if the current cipher is an AEAD cipher
    #[cfg(feature = "v1-aead")]
    pub fn is_aead(&self) -> bool {
        use self::CipherKind::*;

        match *self {
            AES_128_GCM | AES_256_GCM | CHACHA20_POLY1305 => true,

            #[cfg(feature = "v1-aead-extra")]
            AES_128_GCM_SIV | AES_256_GCM_SIV | XCHACHA20_POLY1305 => true,

            _ => false,
        }
    }

    /// Key length of the cipher
    pub fn key_len(&self) -> usize {
        use self::CipherKind::*;

        match *self {
            NONE => 0,

            #[cfg(feature = "v1-stream")]
            SS_TABLE => 0,
            #[cfg(feature = "v1-stream")]
            SS_RC4_MD5 => Rc4Md5::key_size(),

            #[cfg(feature = "v1-stream")]
            AES_128_CTR => Aes128Ctr::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_192_CTR => Aes192Ctr::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_256_CTR => Aes256Ctr::KEY_LEN,

            #[cfg(feature = "v1-stream")]
            AES_128_CFB1 => Aes128Cfb1::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_128_CFB8 => Aes128Cfb8::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_128_CFB128 => Aes128Cfb128::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_192_CFB1 => Aes192Cfb1::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_192_CFB8 => Aes192Cfb8::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_192_CFB128 => Aes192Cfb128::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_256_CFB1 => Aes256Cfb1::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_256_CFB8 => Aes256Cfb8::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_256_CFB128 => Aes256Cfb128::KEY_LEN,

            #[cfg(feature = "v1-stream")]
            AES_128_OFB => Aes128Ofb::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_192_OFB => Aes192Ofb::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            AES_256_OFB => Aes256Ofb::KEY_LEN,

            #[cfg(feature = "v1-stream")]
            CAMELLIA_128_CTR => Camellia128Ctr::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_192_CTR => Camellia192Ctr::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_256_CTR => Camellia256Ctr::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_128_CFB1 => Camellia128Cfb1::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_128_CFB8 => Camellia128Cfb8::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_128_CFB128 => Camellia128Cfb128::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_192_CFB1 => Camellia192Cfb1::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_192_CFB8 => Camellia192Cfb8::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_192_CFB128 => Camellia192Cfb128::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_256_CFB1 => Camellia256Cfb1::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_256_CFB8 => Camellia256Cfb8::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_256_CFB128 => Camellia256Cfb128::KEY_LEN,

            #[cfg(feature = "v1-stream")]
            CAMELLIA_128_OFB => Camellia128Ofb::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_192_OFB => Camellia192Ofb::KEY_LEN,
            #[cfg(feature = "v1-stream")]
            CAMELLIA_256_OFB => Camellia256Ofb::KEY_LEN,

            // NOTE: RC4 密码本身支持 1..256 长度的 Key，
            //       但是 SS 这里把 Key 的长度限制在 16.
            #[cfg(feature = "v1-stream")]
            RC4 => Rc4::key_size(),
            #[cfg(feature = "v1-stream")]
            CHACHA20 => Chacha20::key_size(),

            // AEAD
            #[cfg(feature = "v1-aead")]
            AES_128_GCM => Aes128Gcm::key_size(),
            #[cfg(feature = "v1-aead")]
            AES_256_GCM => Aes256Gcm::key_size(),

            #[cfg(feature = "v1-aead-extra")]
            AES_128_GCM_SIV => Aes128GcmSiv::key_size(),
            #[cfg(feature = "v1-aead-extra")]
            AES_256_GCM_SIV => Aes256GcmSiv::key_size(),

            #[cfg(feature = "v1-aead")]
            CHACHA20_POLY1305 => ChaCha20Poly1305::key_size(),

            #[cfg(feature = "v1-aead-extra")]
            XCHACHA20_POLY1305 => XChaCha20Poly1305::key_size(),
        }
    }

    /// Stream Cipher's initializer vector length
    #[cfg(feature = "v1-stream")]
    pub fn iv_len(&self) -> usize {
        use self::CipherKind::*;

        match *self {
            NONE => 0,
            SS_TABLE => 0,

            SS_RC4_MD5 => Rc4Md5::nonce_size(),

            AES_128_CTR => Aes128Ctr::IV_LEN,
            AES_192_CTR => Aes192Ctr::IV_LEN,
            AES_256_CTR => Aes256Ctr::IV_LEN,

            AES_128_CFB1 => Aes128Cfb1::IV_LEN,
            AES_128_CFB8 => Aes128Cfb8::IV_LEN,
            AES_128_CFB128 => Aes128Cfb128::IV_LEN,
            AES_192_CFB1 => Aes192Cfb1::IV_LEN,
            AES_192_CFB8 => Aes192Cfb8::IV_LEN,
            AES_192_CFB128 => Aes192Cfb128::IV_LEN,
            AES_256_CFB1 => Aes256Cfb1::IV_LEN,
            AES_256_CFB8 => Aes256Cfb8::IV_LEN,
            AES_256_CFB128 => Aes256Cfb128::IV_LEN,

            AES_128_OFB => Aes128Ofb::IV_LEN,
            AES_192_OFB => Aes192Ofb::IV_LEN,
            AES_256_OFB => Aes256Ofb::IV_LEN,

            CAMELLIA_128_CTR => Camellia128Ctr::IV_LEN,
            CAMELLIA_192_CTR => Camellia192Ctr::IV_LEN,
            CAMELLIA_256_CTR => Camellia256Ctr::IV_LEN,

            CAMELLIA_128_CFB1 => Camellia128Cfb1::IV_LEN,
            CAMELLIA_128_CFB8 => Camellia128Cfb8::IV_LEN,
            CAMELLIA_128_CFB128 => Camellia128Cfb128::IV_LEN,
            CAMELLIA_192_CFB1 => Camellia192Cfb1::IV_LEN,
            CAMELLIA_192_CFB8 => Camellia192Cfb8::IV_LEN,
            CAMELLIA_192_CFB128 => Camellia192Cfb128::IV_LEN,
            CAMELLIA_256_CFB1 => Camellia256Cfb1::IV_LEN,
            CAMELLIA_256_CFB8 => Camellia256Cfb8::IV_LEN,
            CAMELLIA_256_CFB128 => Camellia256Cfb128::IV_LEN,

            CAMELLIA_128_OFB => Camellia128Ofb::IV_LEN,
            CAMELLIA_192_OFB => Camellia192Ofb::IV_LEN,
            CAMELLIA_256_OFB => Camellia256Ofb::IV_LEN,

            RC4 => Rc4::nonce_size(),
            CHACHA20 => Chacha20::nonce_size(),

            _ => panic!("only support Stream ciphers"),
        }
    }

    /// AEAD Cipher's TAG length
    #[cfg(feature = "v1-aead")]
    pub fn tag_len(&self) -> usize {
        use self::CipherKind::*;

        match *self {
            AES_128_GCM => Aes128Gcm::tag_size(),
            AES_256_GCM => Aes256Gcm::tag_size(),

            #[cfg(feature = "v1-aead-extra")]
            AES_128_GCM_SIV => Aes128GcmSiv::tag_size(),
            #[cfg(feature = "v1-aead-extra")]
            AES_256_GCM_SIV => Aes256GcmSiv::tag_size(),

            CHACHA20_POLY1305 => ChaCha20Poly1305::tag_size(),

            #[cfg(feature = "v1-aead-extra")]
            XCHACHA20_POLY1305 => XChaCha20Poly1305::tag_size(),

            _ => panic!("only support AEAD ciphers"),
        }
    }

    /// AEAD Cipher's SALT length
    #[cfg(feature = "v1-aead")]
    pub fn salt_len(&self) -> usize {
        if !self.is_aead() {
            panic!("only support AEAD ciphers");
        }

        self.key_len()
    }
}

impl core::fmt::Display for CipherKind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(match *self {
            CipherKind::NONE => "none",

            #[cfg(feature = "v1-stream")]
            CipherKind::SS_TABLE => "table",
            #[cfg(feature = "v1-stream")]
            CipherKind::SS_RC4_MD5 => "rc4-md5",

            #[cfg(feature = "v1-stream")]
            CipherKind::AES_128_CTR => "aes-128-ctr",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_192_CTR => "aes-192-ctr",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_256_CTR => "aes-256-ctr",

            #[cfg(feature = "v1-stream")]
            CipherKind::AES_128_CFB128 => "aes-128-cfb",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_128_CFB1 => "aes-128-cfb1",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_128_CFB8 => "aes-128-cfb8",

            #[cfg(feature = "v1-stream")]
            CipherKind::AES_192_CFB128 => "aes-192-cfb",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_192_CFB1 => "aes-192-cfb1",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_192_CFB8 => "aes-192-cfb8",

            #[cfg(feature = "v1-stream")]
            CipherKind::AES_256_CFB128 => "aes-256-cfb",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_256_CFB1 => "aes-256-cfb1",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_256_CFB8 => "aes-256-cfb8",

            #[cfg(feature = "v1-stream")]
            CipherKind::AES_128_OFB => "aes-128-ofb",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_192_OFB => "aes-192-ofb",
            #[cfg(feature = "v1-stream")]
            CipherKind::AES_256_OFB => "aes-256-ofb",

            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_128_CTR => "camellia-128-ctr",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_192_CTR => "camellia-192-ctr",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_256_CTR => "camellia-256-ctr",

            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_128_CFB128 => "camellia-128-cfb",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_128_CFB1 => "camellia-128-cfb1",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_128_CFB8 => "camellia-128-cfb8",

            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_192_CFB128 => "camellia-192-cfb",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_192_CFB1 => "camellia-192-cfb1",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_192_CFB8 => "camellia-192-cfb8",

            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_256_CFB128 => "camellia-256-cfb",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_256_CFB1 => "camellia-256-cfb1",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_256_CFB8 => "camellia-256-cfb8",

            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_128_OFB => "camellia-128-ofb",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_192_OFB => "camellia-192-ofb",
            #[cfg(feature = "v1-stream")]
            CipherKind::CAMELLIA_256_OFB => "camellia-256-ofb",

            #[cfg(feature = "v1-stream")]
            CipherKind::RC4 => "rc4",
            #[cfg(feature = "v1-stream")]
            CipherKind::CHACHA20 => "chacha20-ietf",

            #[cfg(feature = "v1-aead")]
            CipherKind::AES_128_GCM => "aes-128-gcm",
            #[cfg(feature = "v1-aead")]
            CipherKind::AES_256_GCM => "aes-256-gcm",

            #[cfg(feature = "v1-aead-extra")]
            CipherKind::AES_128_GCM_SIV => "aes-128-gcm-siv",
            #[cfg(feature = "v1-aead-extra")]
            CipherKind::AES_256_GCM_SIV => "aes-256-gcm-siv",

            #[cfg(feature = "v1-aead")]
            CipherKind::CHACHA20_POLY1305 => "chacha20-ietf-poly1305",

            #[cfg(feature = "v1-aead-extra")]
            CipherKind::XCHACHA20_POLY1305 => "xchacha20-ietf-poly1305",
        })
    }
}

/// Error while parsing `CipherKind` from string
#[derive(Debug, Clone)]
pub struct ParseCipherKindError;

impl core::fmt::Display for ParseCipherKindError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid CipherKind")
    }
}

impl core::str::FromStr for CipherKind {
    type Err = ParseCipherKindError;

    fn from_str(s: &str) -> Result<Self, ParseCipherKindError> {
        use self::CipherKind::*;

        match s.to_lowercase().as_str() {
            "plain" | "none" => Ok(NONE),

            #[cfg(feature = "v1-stream")]
            "table" | "" => Ok(SS_TABLE),
            #[cfg(feature = "v1-stream")]
            "rc4-md5" => Ok(SS_RC4_MD5),

            #[cfg(feature = "v1-stream")]
            "aes-128-ctr" => Ok(AES_128_CTR),
            #[cfg(feature = "v1-stream")]
            "aes-192-ctr" => Ok(AES_192_CTR),
            #[cfg(feature = "v1-stream")]
            "aes-256-ctr" => Ok(AES_256_CTR),

            #[cfg(feature = "v1-stream")]
            "aes-128-cfb" => Ok(AES_128_CFB128),
            #[cfg(feature = "v1-stream")]
            "aes-128-cfb1" => Ok(AES_128_CFB1),
            #[cfg(feature = "v1-stream")]
            "aes-128-cfb8" => Ok(AES_128_CFB8),
            #[cfg(feature = "v1-stream")]
            "aes-128-cfb128" => Ok(AES_128_CFB128),

            #[cfg(feature = "v1-stream")]
            "aes-192-cfb" => Ok(AES_192_CFB128),
            #[cfg(feature = "v1-stream")]
            "aes-192-cfb1" => Ok(AES_192_CFB1),
            #[cfg(feature = "v1-stream")]
            "aes-192-cfb8" => Ok(AES_192_CFB8),
            #[cfg(feature = "v1-stream")]
            "aes-192-cfb128" => Ok(AES_192_CFB128),

            #[cfg(feature = "v1-stream")]
            "aes-256-cfb" => Ok(AES_256_CFB128),
            #[cfg(feature = "v1-stream")]
            "aes-256-cfb1" => Ok(AES_256_CFB1),
            #[cfg(feature = "v1-stream")]
            "aes-256-cfb8" => Ok(AES_256_CFB8),
            #[cfg(feature = "v1-stream")]
            "aes-256-cfb128" => Ok(AES_256_CFB128),

            #[cfg(feature = "v1-stream")]
            "aes-128-ofb" => Ok(AES_128_OFB),
            #[cfg(feature = "v1-stream")]
            "aes-192-ofb" => Ok(AES_192_OFB),
            #[cfg(feature = "v1-stream")]
            "aes-256-ofb" => Ok(AES_256_OFB),

            #[cfg(feature = "v1-stream")]
            "camellia-128-ctr" => Ok(CAMELLIA_128_CTR),
            #[cfg(feature = "v1-stream")]
            "camellia-192-ctr" => Ok(CAMELLIA_192_CTR),
            #[cfg(feature = "v1-stream")]
            "camellia-256-ctr" => Ok(CAMELLIA_256_CTR),

            #[cfg(feature = "v1-stream")]
            "camellia-128-cfb" => Ok(CAMELLIA_128_CFB128),
            #[cfg(feature = "v1-stream")]
            "camellia-128-cfb1" => Ok(CAMELLIA_128_CFB1),
            #[cfg(feature = "v1-stream")]
            "camellia-128-cfb8" => Ok(CAMELLIA_128_CFB8),
            #[cfg(feature = "v1-stream")]
            "camellia-128-cfb128" => Ok(CAMELLIA_128_CFB128),

            #[cfg(feature = "v1-stream")]
            "camellia-192-cfb" => Ok(CAMELLIA_192_CFB128),
            #[cfg(feature = "v1-stream")]
            "camellia-192-cfb1" => Ok(CAMELLIA_192_CFB1),
            #[cfg(feature = "v1-stream")]
            "camellia-192-cfb8" => Ok(CAMELLIA_192_CFB8),
            #[cfg(feature = "v1-stream")]
            "camellia-192-cfb128" => Ok(CAMELLIA_192_CFB128),

            #[cfg(feature = "v1-stream")]
            "camellia-256-cfb" => Ok(CAMELLIA_256_CFB128),
            #[cfg(feature = "v1-stream")]
            "camellia-256-cfb1" => Ok(CAMELLIA_256_CFB1),
            #[cfg(feature = "v1-stream")]
            "camellia-256-cfb8" => Ok(CAMELLIA_256_CFB8),
            #[cfg(feature = "v1-stream")]
            "camellia-256-cfb128" => Ok(CAMELLIA_256_CFB128),

            #[cfg(feature = "v1-stream")]
            "camellia-128-ofb" => Ok(CAMELLIA_128_OFB),
            #[cfg(feature = "v1-stream")]
            "camellia-192-ofb" => Ok(CAMELLIA_192_OFB),
            #[cfg(feature = "v1-stream")]
            "camellia-256-ofb" => Ok(CAMELLIA_256_OFB),

            #[cfg(feature = "v1-stream")]
            "rc4" => Ok(RC4),
            #[cfg(feature = "v1-stream")]
            "chacha20-ietf" => Ok(CHACHA20),

            // AEAD Ciphers
            #[cfg(feature = "v1-aead")]
            "aes-128-gcm" => Ok(AES_128_GCM),
            #[cfg(feature = "v1-aead")]
            "aes-256-gcm" => Ok(AES_256_GCM),

            #[cfg(feature = "v1-aead-extra")]
            "aes-128-gcm-siv" => Ok(AES_128_GCM_SIV),
            #[cfg(feature = "v1-aead-extra")]
            "aes-256-gcm-siv" => Ok(AES_256_GCM_SIV),

            #[cfg(feature = "v1-aead")]
            "chacha20-ietf-poly1305" => Ok(CHACHA20_POLY1305),

            #[cfg(feature = "v1-aead-extra")]
            "xchacha20-ietf-poly1305" => Ok(XCHACHA20_POLY1305),

            _ => Err(ParseCipherKindError),
        }
    }
}
