pub mod error;
pub mod jwt;
pub mod pwd;

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub use self::error::{Error, Result};

pub fn encrypt_into_b64u(
    key: &[u8],
    pwd_clear: &String
) -> Result<String> {
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    hmac_sha512.update(pwd_clear.as_bytes());

    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    let result = base64_url::encode(&result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rand::RngCore;

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut fx_key);
        let pwd_clear = "hello world".to_string();

        let fx_res = encrypt_into_b64u(&fx_key, &pwd_clear)?;
        
        let res = encrypt_into_b64u(&fx_key, &pwd_clear)?;

        assert_eq!(res, fx_res);

        Ok(())
    }
}