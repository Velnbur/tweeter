use s3::{creds::Credentials, Bucket, Region};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Storage {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Storage {
    pub async fn parse(self) -> Result<s3::Bucket, s3::error::S3Error> {
        Ok(Bucket::new(
            &self.bucket,
            Region::Custom {
                region: "".into(),
                endpoint: self.endpoint.into(),
            },
            Credentials {
                access_key: Some(self.access_key),
                secret_key: Some(self.secret_key),
                security_token: None,
                session_token: None,
                expiration: None,
            },
        )?
        .with_path_style())
    }
}
