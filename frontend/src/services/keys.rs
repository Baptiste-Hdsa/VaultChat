use indexed_db_futures::Build;
use indexed_db_futures::database::Database;
use indexed_db_futures::query_source::QuerySource;
use indexed_db_futures::transaction::TransactionMode;
use js_sys::Promise;
use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::CryptoKey;
use web_sys::window;

fn subtle() -> web_sys::SubtleCrypto {
    window().unwrap().crypto().unwrap().subtle()
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &b in bytes {
        s.push(b as char);
    }
    window().unwrap().btoa(&s).unwrap()
}

pub fn base64_to_bytes(base64: &str) -> Vec<u8> {
    let window = web_sys::window().unwrap();
    let binary_string = window.atob(base64).unwrap();
    binary_string.chars().map(|c| c as u8).collect()
}

pub async fn generate_rsa_keypair() -> Result<(CryptoKey, CryptoKey), JsValue> {
    let alg = Object::new();
    Reflect::set(&alg, &"name".into(), &"RSA-OAEP".into())?;
    Reflect::set(&alg, &"modulusLength".into(), &2048.into())?;

    let pub_exp = Uint8Array::from([0x01, 0x00, 0x01].as_slice());
    Reflect::set(&alg, &"publicExponent".into(), &pub_exp)?;
    Reflect::set(&alg, &"hash".into(), &"SHA-256".into())?;

    let usages = Array::of2(&"encrypt".into(), &"decrypt".into());

    let promise = subtle().generate_key_with_object(&alg, true, &usages.into())?;
    let key_pair_js = JsFuture::from(promise).await?;

    let pub_key: CryptoKey = Reflect::get(&key_pair_js, &"publicKey".into())?.unchecked_into();
    let priv_key: CryptoKey = Reflect::get(&key_pair_js, &"privateKey".into())?.unchecked_into();

    Ok((pub_key, priv_key))
}

pub async fn derive_kek(password: &str, salt: &[u8]) -> Result<CryptoKey, JsValue> {
    let pass_bytes = Uint8Array::from(password.as_bytes());

    let pbkdf2_alg = Object::new();
    Reflect::set(&pbkdf2_alg, &"name".into(), &"PBKDF2".into())?;

    let usages = Array::of1(&"deriveKey".into());
    let base_key_promise =
        subtle().import_key_with_str("raw", &pass_bytes, "PBKDF2", false, &usages.into())?;
    let base_key: CryptoKey = JsFuture::from(base_key_promise).await?.unchecked_into();

    let params = Object::new();
    Reflect::set(&params, &"name".into(), &"PBKDF2".into())?;
    Reflect::set(&params, &"salt".into(), &Uint8Array::from(salt))?;
    Reflect::set(&params, &"iterations".into(), &100_000.into())?;
    Reflect::set(&params, &"hash".into(), &"SHA-256".into())?;

    let target_alg = Object::new();
    Reflect::set(&target_alg, &"name".into(), &"AES-GCM".into())?;
    Reflect::set(&target_alg, &"length".into(), &256.into())?;

    let ext_usages = Array::of2(&"encrypt".into(), &"decrypt".into());

    let derived_key_promise = subtle().derive_key_with_object_and_object(
        &params,
        &base_key,
        &target_alg,
        false,
        &ext_usages.into(),
    )?;

    Ok(JsFuture::from(derived_key_promise).await?.unchecked_into())
}

pub async fn wrap_private_key(
    rsa_private_key: &CryptoKey,
    kek: &CryptoKey,
    iv: &[u8],
) -> Result<Vec<u8>, JsValue> {
    let export_promise = subtle().export_key("pkcs8", rsa_private_key)?;
    let exported_buffer = JsFuture::from(export_promise).await?;

    let aes_params = Object::new();
    Reflect::set(&aes_params, &"name".into(), &"AES-GCM".into())?;
    Reflect::set(&aes_params, &"iv".into(), &Uint8Array::from(iv))?;

    let encrypt_promise = subtle().encrypt_with_object_and_buffer_source(
        &aes_params,
        kek,
        &exported_buffer.into(),
    )?;

    let ciphertext_buffer = JsFuture::from(encrypt_promise).await?;
    Ok(Uint8Array::new(&ciphertext_buffer).to_vec())
}

pub async fn export_public_key(pub_key: &CryptoKey) -> Result<String, JsValue> {
    let export_promise = subtle().export_key("spki", pub_key)?;
    let exported_buffer = JsFuture::from(export_promise).await?;
    Ok(bytes_to_base64(&Uint8Array::new(&exported_buffer).to_vec()))
}

pub async fn unwrap_private_key(
    wrapped_key_bytes: &[u8],
    kek: &CryptoKey,
    iv: &[u8],
) -> Result<CryptoKey, JsValue> {
    let aes_params = Object::new();
    Reflect::set(&aes_params, &"name".into(), &"AES-GCM".into())?;
    Reflect::set(&aes_params, &"iv".into(), &Uint8Array::from(iv))?;

    let decrypt_promise = subtle().decrypt_with_object_and_buffer_source(
        &aes_params,
        kek,
        &Uint8Array::from(wrapped_key_bytes),
    )?;

    let decrypted_buffer = JsFuture::from(decrypt_promise).await?;

    let rsa_alg = Object::new();
    Reflect::set(&rsa_alg, &"name".into(), &"RSA-OAEP".into())?;
    Reflect::set(&rsa_alg, &"hash".into(), &"SHA-256".into())?;

    let subtle_engine = subtle();
    let import_key_fn = Reflect::get(&subtle_engine, &"importKey".into())?;

    let args = Array::of5(
        &"pkcs8".into(),
        &decrypted_buffer,
        &rsa_alg,
        &true.into(),
        &Array::of1(&"decrypt".into()),
    );

    let import_result_jsvalue = Reflect::apply(
        &import_key_fn.unchecked_into::<js_sys::Function>(),
        &subtle_engine,
        &args,
    )?;

    let import_promise: js_sys::Promise = import_result_jsvalue.unchecked_into();

    Ok(JsFuture::from(import_promise).await?.unchecked_into())
}

pub async fn export_private_key(priv_key: &CryptoKey) -> Result<String, JsValue> {
    let export_promise = subtle().export_key("pkcs8", priv_key)?;
    let exported_buffer = JsFuture::from(export_promise).await?;
    Ok(bytes_to_base64(&Uint8Array::new(&exported_buffer).to_vec()))
}

pub async fn save_key_securely(key: &str) -> Result<(), JsValue> {
    // 1. Open the database using the 0.6.x builder API
    let db = Database::open("VaultChatDB")
        .with_version(1u8)
        .with_on_upgrade_needed(|_event, db| {
            // This runs on first boot when version 1 is created.
            // We create the store and ignore the error if it already exists.
            let _ = db.create_object_store("secrets").build();
            Ok(())
        })
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 2. Open a read/write transaction to the 'secrets' table
    let transaction = db
        .transaction("secrets")
        .with_mode(TransactionMode::Readwrite)
        .build()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let store = transaction
        .object_store("secrets")
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 3. Save the Base64 key string
    store
        .put(key)
        .with_key("priv_key")
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 4. CRITICAL: In 0.6.x, transactions roll back by default!
    // You MUST explicitly commit them.
    transaction
        .commit()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}

pub async fn get_key_securely() -> Result<String, JsValue> {
    // 1. Open the existing database
    let db = Database::open("VaultChatDB")
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 2. Open a Read-Only transaction
    let transaction = db
        .transaction("secrets")
        .with_mode(TransactionMode::Readonly)
        .build()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let store = transaction
        .object_store("secrets")
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 3. Fetch the key
    let result: Option<JsValue> = store
        .get("priv_key")
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 4. Convert the JS value back into a Rust String
    if let Some(js_val) = result {
        if let Some(key_str) = js_val.as_string() {
            return Ok(key_str);
        }
    }

    Err(JsValue::from_str("Private key not found in IndexedDB"))
}

pub async fn encrypt_message(public_key_b64: &str, plaintext: &str) -> Result<String, JsValue> {
    if plaintext.is_empty() {
        return Err(JsValue::from_str("Empty message"));
    }

    let pub_key_bytes = base64_to_bytes(public_key_b64);

    // 1. EXACT match for Import Algorithm
    let rsa_alg = Object::new();
    Reflect::set(&rsa_alg, &"name".into(), &"RSA-OAEP".into())?;
    Reflect::set(&rsa_alg, &"hash".into(), &"SHA-256".into())?;

    let subtle_engine = subtle();
    let import_key_fn = Reflect::get(&subtle_engine, &"importKey".into())?;

    let import_args = Array::of5(
        &"spki".into(),
        &Uint8Array::from(pub_key_bytes.as_slice()).into(),
        &rsa_alg,
        &false.into(),
        &Array::of1(&"encrypt".into()),
    );

    let import_promise: js_sys::Promise = Reflect::apply(
        &import_key_fn.unchecked_into::<js_sys::Function>(),
        &subtle_engine,
        &import_args,
    )?
    .unchecked_into();

    let crypto_pub_key: CryptoKey = JsFuture::from(import_promise).await?.unchecked_into();

    // 2. EXACT match for Encryption Algorithm
    let encrypt_alg = Object::new();
    Reflect::set(&encrypt_alg, &"name".into(), &"RSA-OAEP".into())?;
    // Note: Some browsers REQUIRE the hash here too for RSA-OAEP
    Reflect::set(&encrypt_alg, &"hash".into(), &"SHA-256".into())?;

    let text_bytes = Uint8Array::from(plaintext.as_bytes());

    let encrypt_promise = subtle().encrypt_with_object_and_buffer_source(
        &encrypt_alg,
        &crypto_pub_key,
        &text_bytes.into(),
    )?;

    let ciphertext_buffer = JsFuture::from(encrypt_promise).await?;
    Ok(bytes_to_base64(
        &Uint8Array::new(&ciphertext_buffer).to_vec(),
    ))
}

pub async fn decrypt_message(
    private_key_b64: &str,
    encrypted_b64: &str,
) -> Result<String, JsValue> {
    let priv_key_bytes = base64_to_bytes(private_key_b64);
    let ciphertext_bytes = base64_to_bytes(encrypted_b64);

    let rsa_alg = Object::new();
    Reflect::set(&rsa_alg, &"name".into(), &"RSA-OAEP".into())?;
    Reflect::set(&rsa_alg, &"hash".into(), &"SHA-256".into())?;

    let subtle_engine = subtle();
    let import_key_fn = Reflect::get(&subtle_engine, &"importKey".into())?;

    let args = Array::of5(
        &"pkcs8".into(),
        &Uint8Array::from(priv_key_bytes.as_slice()).into(),
        &rsa_alg.clone(),
        &false.into(),
        &Array::of1(&"decrypt".into()),
    );

    let import_result = Reflect::apply(
        &import_key_fn.unchecked_into::<js_sys::Function>(),
        &subtle_engine,
        &args,
    )?;
    let import_promise: Promise = import_result.unchecked_into();
    let crypto_priv_key: CryptoKey = JsFuture::from(import_promise).await?.unchecked_into();

    let decrypt_alg = Object::new();
    Reflect::set(&decrypt_alg, &"name".into(), &"RSA-OAEP".into())?;

    let decrypt_promise = subtle().decrypt_with_object_and_buffer_source(
        &decrypt_alg,
        &crypto_priv_key,
        &Uint8Array::from(ciphertext_bytes.as_slice()).into(),
    )?;

    let decrypted_buffer = JsFuture::from(decrypt_promise).await?;
    let decrypted_bytes = Uint8Array::new(&decrypted_buffer).to_vec();

    String::from_utf8(decrypted_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
}
