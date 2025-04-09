// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use azure_core::{create_enum, create_extensible_enum};

create_extensible_enum!(
    #[doc = r#"/// Elliptic curve name. For valid values, see JsonWebKeyCurveName."#]
    CurveName,
    #[doc = r#"/// The NIST P-256 elliptic curve, AKA SECG curve SECP256R1."#]
    (P256, "P-256"),
    #[doc = r#"/// The SECG SECP256K1 elliptic curve."#]
    (P256K, "P-256K"),
    #[doc = r#"/// The NIST P-384 elliptic curve, AKA SECG curve SECP384R1."#]
    (P384, "P-384"),
    #[doc = r#"/// The NIST P-521 elliptic curve, AKA SECG curve SECP521R1."#]
    (P521, "P-521")
);

create_extensible_enum!(
    #[doc = r#"/// Reflects the deletion recovery level currently in effect for certificates in the current vault. If it contains 'Purgeable',
/// the certificate can be permanently deleted by a privileged user; otherwise, only the system can purge the certificate,
/// at the end of the retention interval."#]
    DeletionRecoveryLevel,
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable without the possibility for immediate and permanent deletion (i.e.
/// purge when 7 <= SoftDeleteRetentionInDays < 90).This level guarantees the recoverability of the deleted entity during
/// the retention interval and while the subscription is still available."#]
    (CustomizedRecoverable, "CustomizedRecoverable"),
    #[doc = r#"/// Denotes a vault and subscription state in which deletion is recoverable, immediate and permanent deletion (i.e. purge)
/// is not permitted, and in which the subscription itself cannot be permanently canceled when 7 <= SoftDeleteRetentionInDays
/// < 90. This level guarantees the recoverability of the deleted entity during the retention interval, and also reflects
/// the fact that the subscription itself cannot be cancelled."#]
    (
        CustomizedRecoverableProtectedSubscription,
        "CustomizedRecoverable+ProtectedSubscription"
    ),
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable, and which also permits immediate and permanent deletion (i.e.
/// purge when 7 <= SoftDeleteRetentionInDays < 90). This level guarantees the recoverability of the deleted entity during
/// the retention interval, unless a Purge operation is requested, or the subscription is cancelled."#]
    (
        CustomizedRecoverablePurgeable,
        "CustomizedRecoverable+Purgeable"
    ),
    #[doc = r#"/// Denotes a vault state in which deletion is an irreversible operation, without the possibility for recovery. This level
/// corresponds to no protection being available against a Delete operation; the data is irretrievably lost upon accepting
/// a Delete operation at the entity level or higher (vault, resource group, subscription etc.)"#]
    (Purgeable, "Purgeable"),
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable without the possibility for immediate and permanent deletion (i.e.
/// purge). This level guarantees the recoverability of the deleted entity during the retention interval(90 days) and while
/// the subscription is still available. System wil permanently delete it after 90 days, if not recovered"#]
    (Recoverable, "Recoverable"),
    #[doc = r#"/// Denotes a vault and subscription state in which deletion is recoverable within retention interval (90 days), immediate
/// and permanent deletion (i.e. purge) is not permitted, and in which the subscription itself cannot be permanently canceled.
/// System wil permanently delete it after 90 days, if not recovered"#]
    (
        RecoverableProtectedSubscription,
        "Recoverable+ProtectedSubscription"
    ),
    #[doc = r#"/// Denotes a vault state in which deletion is recoverable, and which also permits immediate and permanent deletion (i.e.
/// purge). This level guarantees the recoverability of the deleted entity during the retention interval (90 days), unless
/// a Purge operation is requested, or the subscription is cancelled. System wil permanently delete it after 90 days, if not
/// recovered"#]
    (RecoverablePurgeable, "Recoverable+Purgeable")
);

create_extensible_enum!(
    #[doc = r#"/// An algorithm used for encryption and decryption."#]
    EncryptionAlgorithm,
    #[doc = r#"/// 128-bit AES-CBC."#]
    (A128Cbc, "A128CBC"),
    #[doc = r#"/// 128-bit AES-CBC with PKCS padding."#]
    (A128Cbcpad, "A128CBCPAD"),
    #[doc = r#"/// 128-bit AES-GCM."#]
    (A128Gcm, "A128GCM"),
    #[doc = r#"/// 128-bit AES key wrap."#]
    (A128Kw, "A128KW"),
    #[doc = r#"/// 192-bit AES-CBC."#]
    (A192Cbc, "A192CBC"),
    #[doc = r#"/// 192-bit AES-CBC with PKCS padding."#]
    (A192Cbcpad, "A192CBCPAD"),
    #[doc = r#"/// 192-bit AES-GCM."#]
    (A192Gcm, "A192GCM"),
    #[doc = r#"/// 192-bit AES key wrap."#]
    (A192Kw, "A192KW"),
    #[doc = r#"/// 256-bit AES-CBC."#]
    (A256Cbc, "A256CBC"),
    #[doc = r#"/// 256-bit AES-CBC with PKCS padding."#]
    (A256Cbcpad, "A256CBCPAD"),
    #[doc = r#"/// 256-bit AES-GCM."#]
    (A256Gcm, "A256GCM"),
    #[doc = r#"/// 256-bit AES key wrap."#]
    (A256Kw, "A256KW"),
    #[doc = r#"/// CKM AES key wrap."#]
    (CkmAesKeyWrap, "CKM_AES_KEY_WRAP"),
    #[doc = r#"/// CKM AES key wrap with padding."#]
    (CkmAesKeyWrapPad, "CKM_AES_KEY_WRAP_PAD"),
    #[doc = r#"/// [Not recommended] RSAES-PKCS1-V1_5 key encryption, as described in <https://tools.ietf.org/html/rfc3447>. Microsoft recommends
/// using RSA_OAEP_256 or stronger algorithms for enhanced security. Microsoft does *not* recommend RSA_1_5, which is included
/// solely for backwards compatibility. Cryptographic standards no longer consider RSA with the PKCS#1 v1.5 padding scheme
/// secure for encryption."#]
    (RSA1_5, "RSA1_5"),
    #[doc = r#"/// RSAES using Optimal Asymmetric Encryption Padding with a hash function of SHA-256 and a mask generation function of MGF1
/// with SHA-256."#]
    (RsaOAEP256, "RSA-OAEP-256"),
    #[doc = r#"/// [Not recommended] RSAES using Optimal Asymmetric Encryption Padding (OAEP), as described in <https://tools.ietf.org/html/rfc3447>,
/// with the default parameters specified by RFC 3447 in Section A.2.1. Those default parameters are using a hash function
/// of SHA-1 and a mask generation function of MGF1 with SHA-1. Microsoft recommends using RSA_OAEP_256 or stronger algorithms
/// for enhanced security. Microsoft does *not* recommend RSA_OAEP, which is included solely for backwards compatibility.
/// RSA_OAEP utilizes SHA1, which has known collision problems."#]
    (RsaOaep, "RSA-OAEP")
);

create_extensible_enum!(
    #[doc = r#"/// The encryption algorithm to use to protected the exported key material"#]
    KeyEncryptionAlgorithm,
    #[doc = r#"/// The CKM_RSA_AES_KEY_WRAP key wrap mechanism."#]
    (CkmRsaAesKeyWrap, "CKM_RSA_AES_KEY_WRAP"),
    #[doc = r#"/// The RSA_AES_KEY_WRAP_256 key wrap mechanism."#]
    (RsaAesKeyWrap256, "RSA_AES_KEY_WRAP_256"),
    #[doc = r#"/// The RSA_AES_KEY_WRAP_384 key wrap mechanism."#]
    (RsaAesKeyWrap384, "RSA_AES_KEY_WRAP_384")
);

create_extensible_enum!(
    #[doc = r#"/// JSON web key operations. For more information, see JsonWebKeyOperation."#]
    KeyOperation,
    #[doc = r#"/// Indicates that the key can be used to decrypt."#]
    (Decrypt, "decrypt"),
    #[doc = r#"/// Indicates that the key can be used to encrypt."#]
    (Encrypt, "encrypt"),
    #[doc = r#"/// Indicates that the private component of the key can be exported."#]
    (Export, "export"),
    #[doc = r#"/// Indicates that the key can be imported during creation."#]
    (Import, "import"),
    #[doc = r#"/// Indicates that the key can be used to sign."#]
    (Sign, "sign"),
    #[doc = r#"/// Indicates that the key can be used to unwrap another key."#]
    (UnwrapKey, "unwrapKey"),
    #[doc = r#"/// Indicates that the key can be used to verify."#]
    (Verify, "verify"),
    #[doc = r#"/// Indicates that the key can be used to wrap another key."#]
    (WrapKey, "wrapKey")
);

create_enum!(
    #[doc = r#"/// The type of the action. The value should be compared case-insensitively."#]
    KeyRotationPolicyAction,
    #[doc = r#"/// Trigger Event Grid events. Defaults to 30 days before expiry. Key Vault only."#]
    (Notify, "Notify"),
    #[doc = r#"/// Rotate the key based on the key policy."#]
    (Rotate, "Rotate")
);

create_extensible_enum!(
    #[doc = r#"/// JsonWebKey Key Type (kty), as defined in <https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40>."#]
    KeyType,
    #[doc = r#"/// Elliptic Curve."#]
    (EC, "EC"),
    #[doc = r#"/// Elliptic Curve with a private key which is stored in the HSM."#]
    (EcHsm, "EC-HSM"),
    #[doc = r#"/// Octet sequence (used to represent symmetric keys)"#]
    (Oct, "oct"),
    #[doc = r#"/// Octet sequence (used to represent symmetric keys) which is stored the HSM."#]
    (OctHsm, "oct-HSM"),
    #[doc = r#"/// RSA (https://tools.ietf.org/html/rfc3447)"#]
    (RSA, "RSA"),
    #[doc = r#"/// RSA with a private key which is stored in the HSM."#]
    (RsaHsm, "RSA-HSM")
);

create_extensible_enum!(
    #[doc = r#"/// The signing/verification algorithm identifier. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm."#]
    SignatureAlgorithm,
    #[doc = r#"/// ECDSA using P-256 and SHA-256, as described in <https://tools.ietf.org/html/rfc7518>."#]
    (ES256, "ES256"),
    #[doc = r#"/// ECDSA using P-256K and SHA-256, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (ES256K, "ES256K"),
    #[doc = r#"/// ECDSA using P-384 and SHA-384, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (ES384, "ES384"),
    #[doc = r#"/// ECDSA using P-521 and SHA-512, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (ES512, "ES512"),
    #[doc = r#"/// HMAC using SHA-256, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (HS256, "HS256"),
    #[doc = r#"/// HMAC using SHA-384, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (HS384, "HS384"),
    #[doc = r#"/// HMAC using SHA-512, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (HS512, "HS512"),
    #[doc = r#"/// RSASSA-PSS using SHA-256 and MGF1 with SHA-256, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (PS256, "PS256"),
    #[doc = r#"/// RSASSA-PSS using SHA-384 and MGF1 with SHA-384, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (PS384, "PS384"),
    #[doc = r#"/// RSASSA-PSS using SHA-512 and MGF1 with SHA-512, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (PS512, "PS512"),
    #[doc = r#"/// RSASSA-PKCS1-v1_5 using SHA-256, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (RS256, "RS256"),
    #[doc = r#"/// RSASSA-PKCS1-v1_5 using SHA-384, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (RS384, "RS384"),
    #[doc = r#"/// RSASSA-PKCS1-v1_5 using SHA-512, as described in <https://tools.ietf.org/html/rfc7518>"#]
    (RS512, "RS512"),
    #[doc = r#"/// Reserved"#]
    (RSNULL, "RSNULL")
);
