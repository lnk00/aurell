use serde::{Deserialize, Serialize};

// Shared types for authentication

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Org {
    pub id: String,
    pub name: String,
}

// Sign in types
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SigninRequest {
    pub token: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SigninResponse {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[serde(rename = "sessionJwt")]
    pub session_jwt: String,
}

// Magic link types
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SendMagicLinkRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SendMagicLinkResponse {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VerifyMagicLinkRequest {
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VerifyMagicLinkResponse {
    pub token: String,
    pub orgs: Vec<Org>,
}

// Session types
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RevokeSessionRequest {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RevokeSessionResponse {}
