

/* 
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

    pub fn manager(self, client: &deadpool_postgres::Object) -> Manager {
        Manager {
            client,
            account: self,
        }
    }
}

pub struct Manager<'a> {
    client: &'a deadpool_postgres::Object,
    account: Account,
}
*/
/* 
impl Manager<'_> {
    pub const USERNAME_MAX: usize = 16;
    pub const USERNAME_MIN: usize = 3;
    pub const EMAIL_LIMIT: usize = 320;
    pub const PASSWORD_MIN: usize = 7;

    pub fn add_account(&self) -> Result<Response, AccountError> {
        let acc = &self.account;
        let client = &self.client;
        let rt = Runtime::new().unwrap();

        let username_format = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
        let email_format =
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        match () {
            _ if acc.username.len() < Self::USERNAME_MIN => Err(AccountError::USERNAME_TOO_SHORT(acc.username.clone())),
            _ if acc.username.len() > Self::USERNAME_MAX => Err(AccountError::USERNAME_TOO_LONG(acc.username.clone())),
            _ if !username_format.is_match(&acc.username) => Err(AccountError::USERNAME_BAD_FORMAT),
            _ if !email_format.is_match(&acc.email) => Err(AccountError::EMAIL_BAD_FORMAT),
            _ => Ok(()),
        }.unwrap();

        rt.block_on(async {
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
                Ok(_) => Ok({ Response::json(
                    &json!({
                        "status": "Success"
                    })
                ) }),
                Err(er) => Err({
                    println!("{}", er);
                    AccountError::ACCOUNT_CREATION_FAILED //todo add checks to see what was violated like unique constraint = UsernameTaken
                }),
            }
        })
    }
    pub async fn login_account(login: &str, password: &str, client: &Object) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        let username_regex = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
        let mut type_sql = "";
        let rt = Runtime::new().unwrap();

        if username_regex.is_match(login) {
            type_sql = "SELECT uid, username, email, password_hash, password_salt password FROM accounts where USERNAME = $1"
        }
        if email_regex.is_match(login) {
            type_sql = "SELECT uid, username, email, password_hash, password_salt password FROM accounts where EMAIL = $1"
        }
        
        let prepare = client.prepare(type_sql).await.unwrap();
            match client.query(&prepare, &[&login]).await {
                Ok(v) => {
                    println!("{:#?}", v[0]);
                },
                Err(er) => todo!(),
            }

        false
    }
}

pub struct SessionData {
    sid: String,
    uid: String,
    username: String,
    email: String,
}
pub fn routes(request: &Request, client: &Object, session_data: &mut Option<&SessionData>) -> Response {
    router!(request,
        (POST) (/api/account/register) => {
            #[derive(Deserialize)]
            struct Registration {
                username: String,
                email: String,
                password: String,
            }
            let data: Registration = try_or_400!(rouille::input::json_input(request));
            let acc = Account::new(&data.username, &data.email, &data.password);
            let manager = acc.manager(client);
            manager.add_account().unwrap()
        },
        (POST) (/api/account/login) => {
            let data = try_or_400!(post_input!(request, {
                login: String,
                password: String,
            }));
            println!("Login attempt with login {:?} and password {:?}", data.login, data.password);

            Response::empty_404()
        },
        (GET) (/api/account/logout) => {
            Response::empty_404()
        },
        _ => Response::empty_404()
    )
}
*/

/*
        if acc.username.len() < Self::USERNAME_MIN {
            return Err(AccountError::USERNAME_TOO_SHORT(acc.username.clone()));
        }

        if acc.username.len() > Self::USERNAME_MAX {
            return Err(AccountError::USERNAME_TOO_LONG(acc.username.clone()));
        }

        if !username_format.is_match(&acc.username) {
            return Err(AccountError::USERNAME_BAD_FORMAT);
        }

        if !email_format.is_match(&acc.email) {
            return Err(AccountError::EMAIL_BAD_FORMAT);
        }
*/