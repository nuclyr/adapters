//! AWS authentication via STS AssumeRole (least privilege).

use aws_config::SdkConfig;
use nuclyr_adapter_core::{AdapterError, Provider};

/// Create an AWS SDK config by assuming a role via STS.
/// This is the BYOC flow: customer provides a role ARN,
/// Nuclyr assumes it with least-privilege permissions.
pub async fn assume_role(
    role_arn: &str,
    external_id: Option<&str>,
    session_name: &str,
) -> Result<SdkConfig, AdapterError> {
    let base_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let sts_client = aws_sdk_sts::Client::new(&base_config);

    let mut request = sts_client
        .assume_role()
        .role_arn(role_arn)
        .role_session_name(session_name);

    if let Some(eid) = external_id {
        request = request.external_id(eid);
    }

    let creds = request.send().await.map_err(|e| AdapterError::Auth {
        provider: Provider::Aws,
        message: format!("STS AssumeRole failed: {e}"),
    })?;

    let credentials = creds.credentials().ok_or_else(|| AdapterError::Auth {
        provider: Provider::Aws,
        message: "no credentials in AssumeRole response".into(),
    })?;

    let assumed_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(aws_sdk_sts::config::SharedCredentialsProvider::new(
            aws_sdk_s3::config::Credentials::new(
                credentials.access_key_id(),
                credentials.secret_access_key(),
                Some(credentials.session_token.clone()),
                None,
                "nuclyr-assumed-role",
            ),
        ))
        .load()
        .await;

    Ok(assumed_config)
}
