use std::time::{SystemTime, UNIX_EPOCH};

use log::{info, warn};
use rouille::{Request, Response, router};

use crate::quik_utils;

use super::AccountError;

#[derive(Default)]
struct Password {
    hash: Box<[u8]>,
    salt: Box<[u8]>,
}

impl Password {
    pub fn new(password: &str) -> Password {
        let quik_hash = quik_utils::quik_hash(password);
        Password {
            hash: quik_hash.0,
            salt: quik_hash.1,
        }
    }
}

pub struct Account {
    uid: i64,
    username: String,
    email: String,
    password: Password,
}

impl Account {
    pub fn new(username: &str, email: &str, password: &str) -> Account {
        Account {
            uid: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .try_into()
                .unwrap(),
            username: String::from(username),
            email: String::from(email),
            password: Password::new(password),
        }
    }

    pub fn manager(self, client: deadpool_postgres::Object) -> Manager {
        Manager {
            client,
            account: self,
        }
    }
}

pub struct Manager {
    client: deadpool_postgres::Object,
    account: Account,
}

impl Manager {
    pub const USERNAME_MAX: usize = 16;
    pub const USERNAME_MIN: usize = 3;
    pub const EMAIL_LIMIT: usize = 320;
    pub const PASSWORD_MIN: usize = 7;

    pub async fn add_account(&self) -> Result<(), AccountError> {
        let acc = &self.account;
        let client = &self.client;

        if acc.username.len() < Self::USERNAME_MIN {
            return Err(AccountError::USERNAME_TOO_SHORT(acc.username.clone()));
        }

        if acc.username.len() > Self::USERNAME_MAX {
            return Err(AccountError::USERNAME_TOO_LONG(acc.username.clone()));
        }

        let sql = self
            .client
            .prepare(
                "
            INSERT INTO accounts (
                uid, 
                username, 
                email, 
                password_hash, 
                password_salt
            ) VALUES ($1, $2, $3, $4, $5)",
            )
            .await
            .unwrap();
        let result = client
            .execute(
                &sql,
                &[
                    &acc.uid,
                    &acc.username,
                    &acc.email,
                    &acc.password.hash.as_ref(),
                    &acc.password.salt.as_ref(),
                ],
            )
            .await;
        match result {
            Ok(_) => Ok({
                println!(
                    "Successfully created an account with the UID={}",
                    &self.account.uid
                )
            }),
            Err(er) => Err({
                println!("{}", er);
                AccountError::ACCOUNT_CREATION_FAILED //todo add checks to see what was violated like unique constraint = UsernameTaken
            }),
        }
    }
}

pub fn routes(request: &Request) -> Response {
    router!(request, 
        (POST) (/api/account/register) => {
            Response::empty_404()
        },
        (POST) (/api/account/login) => {
            Response::empty_404()
        },
        (GET) (/api/account/logout) => {
            Response::empty_404()
        },
        _ => Response::empty_404()
    )
}
