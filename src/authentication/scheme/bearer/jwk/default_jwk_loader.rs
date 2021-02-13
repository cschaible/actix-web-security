use jsonwebtoken::Algorithm;

use crate::authentication::error::error_type::JwkLoaderError;
use crate::authentication::scheme::bearer::jwk::default_jwk::DefaultJwks;
use crate::authentication::scheme::bearer::jwk::JwkLoader;
use crate::authentication::scheme::bearer::jwt::default_jwt::DefaultJwt;
use crate::authentication::scheme::bearer::jwt::token::decoder::rsa_decoder::RsaJwtDecoder;
use crate::authentication::scheme::bearer::jwt::token::decoder::TokenDecoder;

pub fn load_default_rsa_jwks(
    url: String,
    algorithm: Algorithm,
) -> Result<Vec<Box<dyn TokenDecoder<DefaultJwt>>>, JwkLoaderError> {
    let jwk_loader: JwkLoader<DefaultJwks> = JwkLoader::from_url(url)?;
    let mut jwk_decoders: Vec<Box<dyn TokenDecoder<DefaultJwt>>> = Vec::new();
    for jwk in jwk_loader.jwks.keys {
        jwk_decoders.push(Box::new(RsaJwtDecoder::new(algorithm, vec![Box::new(jwk)])));
    }
    Ok(jwk_decoders)
}
