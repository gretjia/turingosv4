use secrecy::SecretString;
use std::sync::{Mutex, OnceLock};
use tempfile::tempdir;
use turingosv4::bottom_white::ledger::system_keypair::{
    generate_or_load_system_keypair, load_existing_keypair,
};

fn env_lock() -> &'static Mutex<()> {
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

struct EnvGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let old = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(old) = &self.old {
            std::env::set_var(self.key, old);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

#[test]
fn second_boot_load_succeeds_and_wrong_password_fails() {
    let _lock = env_lock().lock().expect("env lock");
    let tmp = tempdir().expect("tempdir");
    let keystore = tmp.path().join("system_keypair_v1.enc");
    let _path = EnvGuard::set(
        "TURINGOS_KEYSTORE_PATH",
        keystore.to_str().expect("utf8 path"),
    );
    let _mem = EnvGuard::set("TURINGOS_KDF_MEMORY_KIB", "64");
    let _iter = EnvGuard::set("TURINGOS_KDF_ITER", "1");
    let _lanes = EnvGuard::set("TURINGOS_KDF_LANES", "1");

    let password = SecretString::new("runtime password".to_string());
    let generated = generate_or_load_system_keypair(&keystore, &password).expect("generate");
    let public_key = generated.public_key();

    let loaded = generate_or_load_system_keypair(&keystore, &password).expect("load existing");
    assert_eq!(loaded.public_key(), public_key);

    let loaded_direct = load_existing_keypair(&keystore, &password).expect("direct load");
    assert_eq!(loaded_direct.public_key(), public_key);

    let wrong = SecretString::new("wrong password".to_string());
    assert!(
        load_existing_keypair(&keystore, &wrong).is_err(),
        "wrong password must fail authenticated decryption"
    );
}
