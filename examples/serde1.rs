use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

const KEY: &[u8] = b"0123456789abcdef0123456789abcdef";

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "privateAge")]
    age: u32,
    date_of_birth: DateTime<Utc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skills: Vec<String>,
    state: WorkState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    #[serde(
        serialize_with = "serialize_encrypt",
        deserialize_with = "deserialize_decrypt"
    )]
    sensitive: String,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    url: Vec<http::Uri>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum WorkState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated,
}

fn main() -> Result<()> {
    // let state = WorkState::Working("Rust Engineer".to_string());
    let state1 = WorkState::OnLeave(Utc::now());
    // let state2 = WorkState::Terminated;
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        date_of_birth: Utc::now(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
        state: state1,
        data: vec![1, 2, 3, 4, 5],
        sensitive: "data".to_string(),
        url: ["https://example.com".parse()?].to_vec(),
    };
    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);
    println!("host: {:?}", user1.url[0].host());
    Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

#[allow(dead_code)]
fn serialize_encrypt<S>(data: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encrypted = encrypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&encrypted)
}

#[allow(dead_code)]
fn deserialize_decrypt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encrypted = String::deserialize(deserializer)?;
    let decrypted = decrypt(&encrypted).map_err(serde::de::Error::custom)?;
    let decrypted = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
    Ok(decrypted)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded.as_bytes())
        .map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

// encrypt with chacha20poly1305 and then encode with base64
fn encrypt(data: &[u8]) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data)?;
    let nonce_cypertext: Vec<_> = nonce.iter().copied().chain(ciphertext).collect();
    let encoded = URL_SAFE_NO_PAD.encode(nonce_cypertext);
    Ok(encoded)
}

fn decrypt(encoded: &str) -> Result<Vec<u8>> {
    let decoded = URL_SAFE_NO_PAD.decode(encoded.as_bytes())?;
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = decoded[..12].into();
    let decrypted = cipher.decrypt(nonce, &decoded[12..]).unwrap();
    Ok(decrypted)
}
