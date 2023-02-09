use std::{env::args, error::Error, path::Path};

use libcryptsetup_rs::{consts::vals::EncryptionFormat, CryptInit, LibcryptErr};

fn main() -> Result<(), Box<dyn Error>> {
    let path = args().nth(1).ok_or_else(|| {
        LibcryptErr::Other("Path for device required as only argument".to_string())
    })?;

    let mut device = CryptInit::init(Path::new(&path))?;
    device.context_handle().format::<()>(
        EncryptionFormat::Luks2,
        ("aes", "xts-plain"),
        None,
        libcryptsetup_rs::Either::Right(256 / 8),
        None,
    )?;
    device
        .context_handle()
        .set_label(Some("label"), Some("subsystem"))?;
    Ok(())
}
