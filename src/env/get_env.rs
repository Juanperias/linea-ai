use dotenv::dotenv;
use std::env::var;

#[derive(Clone, Debug)]
pub struct EnvVars {
    pub replicate_api_key: String,
    pub discord_token: String,
}

pub fn get_env() -> EnvVars {
    dotenv().ok();

    EnvVars {
        discord_token: var("DISCORD_TOKEN").expect("Discord token not found"),
        replicate_api_key: var("REPLICATE_API_KEY").expect("Replicate api key not found"),
    }
}
