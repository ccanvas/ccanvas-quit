use std::collections::HashSet;

use libccanvas::{
    bindings::{self, EventVariant, KeyEvent, Subscription},
    client::{Client, ClientConfig},
    features::{betterserde::KeyCodeBetterSerde, config::CcanvasConfig},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum KeyModifier {
    #[serde(rename = "alt")]
    Alt,
    #[serde(rename = "ctrl")]
    Ctrl,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "any")]
    Any,
}

impl From<KeyModifier> for Vec<bindings::KeyModifier> {
    fn from(val: KeyModifier) -> Self {
        match val {
            KeyModifier::Any => vec![
                bindings::KeyModifier::Ctrl,
                bindings::KeyModifier::Alt,
                bindings::KeyModifier::None,
            ],
            KeyModifier::Alt => vec![bindings::KeyModifier::Alt],
            KeyModifier::Ctrl => vec![bindings::KeyModifier::Ctrl],
            KeyModifier::None => vec![bindings::KeyModifier::None],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct QuitKey {
    #[serde(flatten)]
    pub key: KeyCodeBetterSerde,
    pub modifier: KeyModifier,
}

#[derive(Serialize, Deserialize, CcanvasConfig)]
struct QuitKeys {
    keys: Vec<QuitKey>,
}

impl Default for QuitKeys {
    fn default() -> Self {
        Self {
            keys: vec![QuitKey {
                key: KeyCodeBetterSerde::Char { value: 'q' },
                modifier: KeyModifier::Ctrl,
            }],
        }
    }
}

impl From<QuitKeys> for HashSet<bindings::KeyEvent> {
    fn from(val: QuitKeys) -> Self {
        let mut set = HashSet::new();
        val.keys.into_iter().for_each(|entry| {
            let modifiers: Vec<bindings::KeyModifier> = entry.modifier.into();
            modifiers.iter().for_each(|modifier| {
                set.insert(bindings::KeyEvent::new(entry.key.into(), *modifier));
            })
        });
        set
    }
}

#[tokio::main]
async fn main() {
    let keys: HashSet<KeyEvent> = QuitKeys::load().into();
    let client = Client::new(ClientConfig::default()).await;

    client
        .subscribe_multiple(
            keys.clone()
                .into_iter()
                .map(|entry| Subscription::specific_keypress(entry).with_priority(0))
                .collect(),
        )
        .await;

    loop {
        let event = client.recv().await;
        if let EventVariant::Key(key) = event.get() {
            if keys.contains(key) {
                client.exit().await;
                return;
            }
        }
    }
}
