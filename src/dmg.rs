use aes::Aes128;
use cipher::consts::U16;
use cipher::generic_array::{ArrayLength, GenericArray};
use cipher::{BlockCipher, BlockDecrypt, KeyInit, StreamCipher};
use pbkdf2::pbkdf2;
use random_string::generate;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

const BASE_MOUNT_PATH: &str = "/tmp/";

pub struct Dmg {
    pub dmg_path: String,
    pub mount_path: String,
}

impl Dmg {
    pub fn new(dmg_path: &str) -> Self {
        Self {
            dmg_path: dmg_path.to_string(),
            mount_path: generate_random_mount_path(),
        }
    }

    pub fn attempt_password(&self, password: &str) -> bool {
        // if self.fast_verify_128(password) {
        //     self.full_verify(password)
        // } else {
        //     false
        // }
        self.fast_verify_128(password)
    }

    fn convert_to_key(
        &self,
        password: &str,
    ) -> GenericArray<u8, <Aes128 as NewBlockCipher>::KeySize> {
        let password_bytes = password.as_bytes();
        let salt = [0u8; 8]; // Use a constant salt for simplicity
        let mut key = [0u8; 16]; // 16 bytes = 128 bits

        // Use PBKDF2 with 1000 iterations to derive the key from the password
        pbkdf2::<Hmac<Sha1>>(password_bytes, &salt, 1000, &mut key);

        GenericArray::clone_from_slice(&key)
    }

    fn fast_verify_128(&self, password: &str) -> bool {
        let key_bytes = password.as_bytes();
        let mut file_data = std::fs::read(&self.dmg_path).unwrap();
        let blocks = file_data.chunks_exact_mut(16);

        let iv: GenericArray<u8, U16> = *GenericArray::from_slice(&[0u8; 16]);

        let cipher = Aes128::new(GenericArray::from_slice(key_bytes));
        let mut prev_block: GenericArray<u8, U16> =
            GenericArray::clone_from_slice(&iv);
        for block in blocks {
            let mut block_data = GenericArray::clone_from_slice(block);
            cipher.decrypt_block(&mut block_data);
            block_data
                .iter_mut()
                .zip(prev_block)
                .for_each(|(a, b)| *a ^= b);
            prev_block = GenericArray::clone_from_slice(block);
        }

        true
    }

    fn full_verify(&self, password: &str) -> bool {
        let mut child = Command::new("hdiutil")
            .arg("verify")
            .arg("-stdinpass")
            .arg(&self.dmg_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to execute hdiutil");

        child
            .stdin
            .as_mut()
            .expect("failed to open stdin")
            .write_all(password.as_bytes())
            .expect("failed to write password to stdin");

        let output =
            child.wait_with_output().expect("failed to wait on hdiutil");
        output.status.success()
    }
}

pub fn generate_random_mount_path() -> String {
    format!(
        "{}{}",
        BASE_MOUNT_PATH,
        generate(6, "abcdefghijklmnopqrstuvwxyz")
    )
}
