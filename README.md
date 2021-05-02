# Actix web security

Basic-Auth / OAuth2 easy-to-use authentication modules for actix web.

## Features

* HTTP Authentication with the following authentication schemes
  * Basic Authentication
  * Bearer Authentication
* OAuth2 Resource Server "Auto-Configuration"
* JWK-Downloader to verify JWTs
* JWT verification

## Note: Neither audited nor penetration tested 
This library is provided "as is" without warranties of any kind and is not verified to be secure. 
It has neither been audited to be safe in an audit nor been penetration tested. 
The library was developed to the best of knowledge and belief.
It's in your own responsibility to check the code for potential security issues or bugs and your own decision 
whether you see the code as safe and trustworthy or whether you prefer to not use it.
The library is provided as open-source and the liability of any kind is excluded as described in the licenses
the software is provided under.


## Install
Add the following dependency to your `cargo.toml`.

```toml
actix-web-security = "0.1.0"
```

The following features can be activated:
* `jwk-loader`  
  This feature can be activated to download custom JWKs from an authorization server
  ```toml
  actix-web-security = { version="0.1.0", features = ["jwk-loader"] }
  ```
  
* `jwk-default-loader`  
  This feature can be activated to download `DefaultJwks` from an authorization server.
  ```toml
  actix-web-security = { version="0.1.0", features = ["jwk-default-loader"] }
  ```

Both features require `openssl` to be installed on the system.
The documentation about how to install it can be found [here](https://docs.rs/openssl/0.10.32/openssl/#automatic).

## Samples
Sample applications can be found [here](https://github.com/cschaible/actix-web-security-samples).



## Usage
A concrete type of UserDetails, BasicUserDetailsService or JwtUserDetailsService,


### User Details
`UserDetails` is a marker trait to be implemented by a user entity. The user entity can be
added to the request extensions to inject it in the API endpoints.

```rust
#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub user_id: String,
    pub name: String
}

impl UserDetails for User {}
```


### User Details Service
One of `BasicUserDetailsService` and `JwtUserDetailsService` traits must be implemented to
resolve users for given credentials / JWTs.  

A `JwtUserDetailsService` implementation could look like:
```rust
#[derive(Clone)]
pub struct JwtUserDetailsServiceImpl {
    pub(crate) user_repository: Arc<UserRepository>,
}

#[async_trait]
impl JwtUserDetailsService for JwtUserDetailsServiceImpl {
    #[allow(clippy::borrowed_box)]
    async fn find_user(&self, token: &Box<dyn Claims>) -> Option<Box<dyn UserDetails>> {
        match token.downcast_ref::<DefaultJwt>() {
            Some(claims) => {
                let sub = claims.sub.clone().expect("sub expected");
                let found_user = self.user_repository.find_by_user_id(sub.clone()).await;
                match found_user {
                    Ok(user) => match user {
                        Some(u) => Some(Box::new(u)),
                        None => None,
                    },
                    Err(e) => None
                }
            }
            None => None,
        }
    }
}
```


### Header Extractor
One of `BasicAuthenticationExtractor` or `BearerAuthenticationExtractor` must be configured.

A `BasicAuthenticationExtractor` can be created easily as shown below:
```rust
BasicAuthenticationExtractor::new()
```
In case of the BearerAuthenticationExtractor one or more `TokenDecoder` must be configured as well. 
If the crate feature `jwk-default-loader` is used the JWKs can be downloaded automatically and token
decoders instantiated automatically by using the `load_default_rsa_jwks` function.

```rust
BearerAuthenticationExtractor::new(load_default_rsa_jwks(auth_server_jwks_url, Algorithm::RS256)?);
```


### Endpoint Matcher
The credentials extraction and authentication can be limited to specific endpoints or applied
to all endpoints. A `EndpointMatcher` must be instantiated. There are two default implementations
available: `AllEndpointsMatcher` to protect all endpoints and `SpecificUrlsMatcher` to protect
the URS with the exact matching URLs. Custom ones can be implemented if the defaults are not
applicable for the use-case.  

```rust
AllUrlMatcher::new()
```
<b>Warning: Endpoints are only protected if the matcher covered the endpoints.</b>


### Authentication Provider
An `AuthenticationProvider` is an abstraction that is used to do the authentication.
There are two default implementations `BasicAuthenticationProvider` and `JwtAuthenticationProvider`.
A custom implementation can be written to use different authentication mechanisms.

A `BasicAuthenticationProvider` can be instantiated as easy as:
```rust
BasicAuthenticationProvider::new(Box::new(user_details_service))
```

A `JwtAuthenticationProvider` can be instantiated as easy as:
```rust
JwtAuthenticationProvider::new(Box::new(user_details_service))
```

### Provider Manager
One or more `AuthenticationProvider` must be configured to authenticate users.
They are registered in a `ProviderManager`.

```rust
ProviderManager::new(vec![authentication_provider1, authentication_provider2])
```

### Middleware
The `HttpAuthenticationModdleware` is the wrapper of all previously described 
components that handles the actual authentication process.

```rust
HttpAuthenticationMiddleware::new(
    ProviderManager::new(vec![
        Box::new(JwtAuthenticationProvider::new(
          Box::new(user_details_service)
        ))
    ]),
    Box::new(authentication_extractor),
    Box::new(endpoint_matcher),
)
```

The middleware can be registered normal in actix:
```rust
HttpServer::new(move || {
    let cors_middleware = ...;
    let auth_middleware = ...;
    App::new()
        .wrap(auth_middleware)
        .wrap(cors_middleware)
        .service(api::endpoint1)
})
.bind("0.0.0.0:8081")?
.run()
.await?;
```

More details can be found in the sample applications [repository](https://github.com/cschaible/actix-web-security-samples).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0])
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT])

at your option.