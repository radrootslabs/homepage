#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;
#[cfg(target_arch = "wasm32")]
use nostr_browser_signer::prelude::*;

#[cfg(target_arch = "wasm32")]
const PUBLIC_KEY_RETRY_COUNT: usize = 20;
#[cfg(target_arch = "wasm32")]
const PUBLIC_KEY_RETRY_DELAY_MS: u32 = 150;

#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
pub async fn load_public_key() -> Option<String> {
    None
}
