use serde::Deserialize;

use crate::authentication::scheme::bearer::jwt::token::decoder::rsa_decoder::RsaKeyComponents;

#[derive(Deserialize, Debug, Clone)]
pub struct DefaultJwks {
    pub keys: Vec<DefaultJwk>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DefaultJwk {
    #[serde(rename = "kty")]
    pub key_type: String,
    #[serde(rename = "use")]
    pub key_use: Option<String>,
    #[serde(rename = "key_ops")]
    pub key_ops: Option<String>,
    #[serde(rename = "alg")]
    pub algorithm: Option<String>,
    #[serde(rename = "kid")]
    pub key_id: Option<String>,
    #[serde(rename = "x5u")]
    pub x509_url: Option<String>,
    #[serde(rename = "x5c")]
    pub x509_chain: Option<Vec<String>>,
    #[serde(rename = "x5t")]
    pub x509_sha1_thumbprint: Option<String>,
    #[serde(rename = "x5t#S256")]
    pub x509_sha256_thumbprint: Option<String>,
    pub e: Option<String>,
    pub n: Option<String>,
}

impl RsaKeyComponents for DefaultJwk {
    fn get_n(&self) -> String {
        self.n.clone().expect("rsa 'n' component not present in jwk")
    }

    fn get_e(&self) -> String {
        self.e.clone().expect("rsa 'e' component not present in jwk")
    }
}
