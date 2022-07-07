use crate::types::*;
use ic_cdk::{call, export::Principal};

const KEY_NAME: &str = "test";

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn ecdsa_public_key(derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // TODO: Replace this principal with the management canister when it's available.
    // For now, call a canister that provides a mock implementation.
    let ecdsa_canister_id = Principal::from_text("b5mls-nqaaa-aaaal-aa2oq-cai").unwrap();

    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.
    let res: (ECDSAPublicKeyReply,) = call(
        ecdsa_canister_id,
        "ecdsa_public_key",
        (ECDSAPublicKey {
            canister_id: None,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: String::from(KEY_NAME),
            },
        },),
    )
    .await
    .unwrap();

    res.0.public_key
}

pub async fn sign_with_ecdsa(derivation_path: Vec<Vec<u8>>, message_hash: Vec<u8>) -> Vec<u8> {
    // TODO: Replace this principal with the management canister when it's available.
    // For now, call a canister that provides a mock implementation.
    let ecdsa_canister_id = Principal::from_text("b5mls-nqaaa-aaaal-aa2oq-cai").unwrap();

    let res: (SignWithECDSAReply,) = call(
        ecdsa_canister_id,
        "sign_with_ecdsa",
        (SignWithECDSA {
            message_hash,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: String::from(KEY_NAME),
            },
        },),
    )
    .await
    .unwrap();

    res.0.signature
}
