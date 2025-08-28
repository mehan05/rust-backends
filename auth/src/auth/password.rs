use argon2::{password_hash::{rand_core::OsRng, SaltString,PasswordHash,PasswordHasher,PasswordVerifier,}, Argon2};



pub fn hash_password(plain: &str) -> Result<(),String >{
    let mut salt= SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(plain.as_bytes(),&salt).unwrap();
    println!("salt: {}",hash);

    Ok(())
    }

pub fn verify_password(plain: &str, hash: &str) -> bool{
        let hash = PasswordHash::new(hash).unwrap();
        let argon2 = Argon2::default();
        let is_match = argon2.verify_password(plain.as_bytes(), &hash).is_ok();
        println!("{:?}",is_match);
        is_match
}