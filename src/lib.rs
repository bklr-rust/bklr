// #[cfg(asynchronous)]
pub mod asynchronous {
    use reqwest::{Url, Client, ClientBuilder, header::HeaderMap};
    use serde::Deserialize;
    // #[cfg(async_login)]
    pub mod login {
        use super::TokenPair;

        pub async fn acquire_token_pair(username: String, password: String, url: reqwest::Url) -> TokenPair {         
            let response = reqwest::Client::new()
            .post(url)
            .send()
            .await;

            let token_pair: TokenPair = match response {
                Ok(response) => {
                    let json: serde_json::Value = response.json().await.unwrap();
                    let token_pair = serde_json::from_value(json).unwrap();
                    token_pair
                },
                Err(e) => panic!("acquire_token_pair(): panicked with {}", e)
            };

            token_pair
        }
    }

    /// Stores the refresh and access API tokens.
   #[derive(Debug, Deserialize)] 
    pub struct TokenPair {
        access_token: String,
        refresh_token: String 
    }   

    /// Structure of the API client. Do not construct it directly! Use the build_bklrclient function instead.
    pub struct BklrClient {
        pub token_pair: TokenPair,
        pub url: Url,
        http_client: Client
    }

    /// Use this function to build an API client. This is the preferred method of constructing a client; Constructing one manually is possible,
    /// and it may be needed in some cases. For example, if the school uses a different server configuration and requires some specialty parameters,
    /// for example an extra HTTP header, manual construction might be needed. This will probably be addressed in later library versions, this issue particularly.
    //TODO: easy custom header support 
    /// # Panics
    /// The client builder will panic if the access token provided has the wrong format, or if reqwest panics, e.g. if it can't initiate a TLS backend.
    pub fn build_bklrclient(access_token: String, refresh_token: String, url: Url) -> BklrClient {
        let useragent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let mut headers = HeaderMap::new();
        headers.append("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
        headers.append("Authorization", format!("Bearer {}", access_token).parse().unwrap()); 
        let reqwest_client = ClientBuilder::new()
            .user_agent(useragent)
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();
        let token_pair = TokenPair { access_token: access_token, refresh_token: refresh_token };

        BklrClient { token_pair: token_pair, url: url, http_client: reqwest_client}
    }

    impl BklrClient {
        pub async fn get_marks(&self) -> serde_json::Value {
            let response = self.http_client.get(format!("{}/api/3/marks", self.url)).send().await.unwrap().json::<serde_json::Value>().await.unwrap();
            response
        }
    }
}