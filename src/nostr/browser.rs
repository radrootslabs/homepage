use gloo_timers::future::TimeoutFuture;
use nostr_browser_signer::prelude::*;

const PUBLIC_KEY_RETRY_COUNT: usize = 20;
const PUBLIC_KEY_RETRY_DELAY_MS: u32 = 150;

pub async fn load_public_key() -> Option<String> {
    for attempt in 0..PUBLIC_KEY_RETRY_COUNT {
        match BrowserSigner::new() {
            Ok(signer) => {
                let key = signer.get_public_key_async().await.ok()?;
                return Some(key.to_hex());
            }
            Err(_) if attempt + 1 < PUBLIC_KEY_RETRY_COUNT => {
                TimeoutFuture::new(PUBLIC_KEY_RETRY_DELAY_MS).await;
            }
            Err(_) => return None,
        }
    }

    None
}
