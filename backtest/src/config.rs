const BUCKET_NAME: &str = "m-etl-bucket-dev";
const SERVICE_ACCOUNT_PATH: &str = "./service-account.json";

/// Backtesting for strategies.
#[derive(argh::FromArgs, Debug)]
pub struct Args {
    /// GCS bucket to fetch historical pool data from
    #[argh(option, default = "BUCKET_NAME.to_string()")]
    pub bucket: String,

    /// path to file holding service account information
    #[argh(option, default = "SERVICE_ACCOUNT_PATH.to_string()")]
    pub service_account: String,
}
