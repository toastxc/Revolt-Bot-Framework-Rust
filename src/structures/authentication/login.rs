use serde::{Deserialize, Serialize};

use super::{
    mfa::{MFAMethod, MFAResponse},
    session::Session,
};

/// # Login Data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]

pub enum DataLogin {
    Email {
        /// Email
        email: String,
        /// Password
        password: String,
        /// Friendly name used for the session
        friendly_name: Option<String>,
    },
    MFA {
        /// Unvalidated or authorised MFA ticket
        ///
        /// Used to resolve the correct account
        mfa_ticket: String,
        /// Valid MFA response
        ///
        /// This will take precedence over the `password` field where applicable
        mfa_response: Option<MFAResponse>,
        /// Friendly name used for the session
        friendly_name: Option<String>,
    },
}

impl DataLogin {
    pub fn mfa(mfa_ticket: &str) -> Self {
        Self::MFA {
            mfa_ticket: String::from(mfa_ticket),
            mfa_response: None,
            friendly_name: None,
        }
    }
    pub fn non_mfa(email: &str, password: &str) -> Self {
        Self::Email {
            email: String::from(email),
            password: String::from(password),
            friendly_name: None,
        }
    }
    pub fn set_friendly_name(&mut self, name: &str) -> Self {
        match self {
            DataLogin::Email { friendly_name, .. } | DataLogin::MFA { friendly_name, .. } => {
                *friendly_name = Some(String::from(name))
            }
        };
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "result")]
pub enum ResponseLogin {
    Success(Session),
    MFA {
        ticket: String,
        allowed_methods: Vec<MFAMethod>,
    },
    Disabled {
        user_id: String,
    },
}
