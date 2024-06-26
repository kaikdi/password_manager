// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use num_bigint::{BigUint, RandomBits};
use rand::{rngs::OsRng, Rng};
use srp::groups::G_4096;

fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn hash_password<'a>(
    password: &'a str,
    salt: &'a SaltString,
) -> Result<argon2::PasswordHash<'a>, argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    argon2.hash_password(password.as_bytes(), salt)
}

#[tauri::command]
fn create_salt_and_verifier(password: &str) -> (String, String) {
    let salt = generate_salt();
    let g = &G_4096.g;
    let n = &G_4096.n;
    let password_hash = hash_password(password, &salt)
        .unwrap()
        .hash
        .unwrap()
        .as_bytes()
        .to_owned();
    let password_hash_big_uint = BigUint::from_bytes_be(&password_hash);
    let v = g.modpow(&password_hash_big_uint, n);

    (salt.to_string(), v.to_string())
}

#[tauri::command]
fn compute_client_value_a() -> String {
    let g = &G_4096.g;
    let n = &G_4096.n;
    let mut rng = rand::thread_rng();
    let private_a: BigUint = rng.sample(RandomBits::new(256));
    let public_a = g.modpow(&private_a, n);

    public_a.to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_salt_and_verifier,
            compute_client_value_a
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
