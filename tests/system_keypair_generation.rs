use secrecy::SecretString;
use std::sync::{Mutex, OnceLock};
use tempfile::tempdir;
use turingosv4::bottom_white::ledger::system_keypair::{
    generate_or_load_system_keypair, SystemEpoch,
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
fn first_boot_keypair_generated_encrypted_at_rest_and_0600() {
    let _lock = env_lock().lock().expect("env lock");
    let tmp = tempdir().expect("tempdir");
    let keystore = tmp.path().join("keystore").join("system_keypair_v1.enc");
    let _path = EnvGuard::set(
        "TURINGOS_KEYSTORE_PATH",
        keystore.to_str().expect("utf8 path"),
    );
    let _mem = EnvGuard::set("TURINGOS_KDF_MEMORY_KIB", "64");
    let _iter = EnvGuard::set("TURINGOS_KDF_ITER", "1");
    let _lanes = EnvGuard::set("TURINGOS_KDF_LANES", "1");

    let password = SecretString::new("correct horse battery staple".to_string());
    let keypair =
        generate_or_load_system_keypair(&keystore, &password).expect("first boot keypair");

    assert!(keystore.exists(), "keystore created");
    let bytes = std::fs::read(&keystore).expect("read keystore");
    assert!(bytes.len() > 64, "encrypted envelope, not raw keypair");
    assert!(
        !bytes
            .windows(32)
            .any(|window| window == keypair.public_key().as_bytes()),
        "public key is inside authenticated ciphertext, not cleartext envelope"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&keystore)
            .expect("metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600, "keystore permissions");
    }

    let default_path =
        turingosv4::bottom_white::ledger::system_keypair::default_system_keystore_path(
            SystemEpoch::new(1),
        )
        .expect("env override path");
    assert_eq!(default_path, keystore);
}
