use std::fs::{remove_file, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use loopdev::LoopControl;
use rand::random;

use crate::err::LibcryptErr;

fn setup_backing_file(size_in_bytes: usize, with_zeros: bool) -> Result<PathBuf, io::Error> {
    let mut i = 0;

    let b64_string = base64::encode_config(
        &random::<[u8; 12]>(),
        base64::Config::new(base64::CharacterSet::UrlSafe, false),
    );
    let mut file_path = PathBuf::new();
    file_path.push(Path::new(&format!("/tmp/{}", b64_string)));

    let mut f = File::create(&file_path)?;
    while i < size_in_bytes {
        let len = if with_zeros {
            f.write(&[0; 4096])?
        } else {
            let buf: Vec<_> = (0..4096).map(|_| random::<u8>()).collect();
            f.write(&buf)?
        };
        assert_eq!(len, 4096);
        i += len;
    }
    Ok(file_path)
}

pub fn use_loopback<F>(
    file_size: usize,
    with_zeros: bool,
    cleanup: bool,
    func: F,
) -> Result<(), LibcryptErr>
where
    F: Fn(&Path, &Path) -> Result<(), LibcryptErr>,
{
    if !nix::unistd::Uid::effective().is_root() {
        panic!("Must be root to run tests");
    }
    let ctrl = LoopControl::open();
    let dev = ctrl
        .and_then(|ref c| c.next_free())
        .map_err(LibcryptErr::IOError)?;

    let path = setup_backing_file(file_size, with_zeros).map_err(LibcryptErr::IOError)?;
    let attach_result = dev.attach_file(&path);
    let test_result = attach_result
        .map_err(LibcryptErr::IOError)
        .and_then(|_| match dev.path() {
            Some(ref d) => func(d, &path),
            _ => Err(LibcryptErr::IOError(io::Error::from(
                io::ErrorKind::NotFound,
            ))),
        });
    dev.detach()
        .and_then(|_| if cleanup { remove_file(&path) } else { Ok(()) })
        .map_err(LibcryptErr::IOError)
        .and(test_result)
}
