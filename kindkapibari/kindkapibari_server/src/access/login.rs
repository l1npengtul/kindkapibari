use std::sync::Arc;
use chrono::{TimeZone, Utc};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use kindkapibari_core::snowflake::SnowflakeIdGenerator;

const AUTH_REDIS_KEY_START_OAUTH: [u8; 6] = *b"kkb:oa";
const AUTH_REDIS_KEY_START_SESSION: [u8; 6] = *b"kkb:se";
const API_KEY_PREFIX_NO_DASH: &'static str = "A";
const TOKEN_PREFIX_NO_DASH: &'static str = "S";

static AUTO_RESEEDING_APIKEY_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_SESSION_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

fn generate_session_key()
