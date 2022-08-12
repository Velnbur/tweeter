use s3::{creds::Credentials, Bucket, Region};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Storage {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Into<s3::Bucket> for Storage {
    fn into(self) -> s3::Bucket {
        Bucket::new(
            &self.bucket,
            Region::Custom {
                region: "".into(),
                endpoint: self.endpoint.into(),
            },
            Credentials {
                access_key: self.access_key.into(),
                secret_key: self.secret_key.into(),
                security_token: None,
                session_token: None,
                expiration: None,
            },
        )
        .expect("failed to parse storage config")
        .with_path_style()
    }
}
