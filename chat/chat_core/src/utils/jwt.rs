use crate::User;
use jwt_simple::prelude::*;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7; // 7 days
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

pub struct EncodingKey(Ed25519KeyPair);

#[allow(unused)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, jwt_simple::Error> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            // time_tolerance 验证令牌时间戳时可容忍多少时钟漂移 默认值为 15 分钟，以解决时钟不完全准确的常见问题
            // time_tolerance: Some(Duration::from_secs(10)), // 测试：这里我们修改为10s，为了快速让token过期
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(opts))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;
        let user = User::new(1, "startdusk", "startdusk@acme.org");

        let token = ek.sign(user.clone())?;
        let decode_user = dk.verify(&token)?;
        assert_eq!(user, decode_user);
        Ok(())
    }
}
