use crate::{Infallible, Signer, SignerInteractivityContext, SigningKey, VerifyingKey};

use async_trait::async_trait;
use scfx_starknet_core::{
    crypto::{EcdsaSignError, Signature},
    types::Felt,
};

/// A signer that simply holds the signing (private) key in memory for performing cryptographic
/// operations. It's recommended to use hardware-based signers for use cases involving real value.
#[derive(Debug, Clone)]
pub struct LocalWallet {
    private_key: SigningKey,
}

/// Errors using [`LocalWallet`].
#[derive(Debug, thiserror::Error)]
pub enum SignError {
    /// ECDSA signature error.
    #[error(transparent)]
    EcdsaSignError(EcdsaSignError),
}

impl LocalWallet {
    /// Constructs [`LocalWallet`] from a [`SigningKey`].
    pub fn from_signing_key(key: SigningKey) -> Self {
        key.into()
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Signer for LocalWallet {
    type GetPublicKeyError = Infallible;
    type SignError = SignError;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError> {
        Ok(self.private_key.verifying_key())
    }

    async fn sign_hash(&self, hash: &Felt) -> Result<Signature, Self::SignError> {
        Ok(self.private_key.sign(hash)?)
    }

    fn is_interactive(&self, _context: SignerInteractivityContext<'_>) -> bool {
        false
    }
}

impl From<SigningKey> for LocalWallet {
    fn from(value: SigningKey) -> Self {
        Self { private_key: value }
    }
}

impl From<EcdsaSignError> for SignError {
    fn from(value: EcdsaSignError) -> Self {
        Self::EcdsaSignError(value)
    }
}
