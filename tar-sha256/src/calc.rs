use std::error::Error;
use std::fs::File;
use std::io::{self, stdin, Read, Stdin};

use sha2::{Digest, Sha256};
use tar::{Archive, EntryType};

enum Read1 {
    F(File),
    S(Stdin),
}

impl Read for Read1 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::F(r) => r.read(buf),
            Self::S(r) => r.read(buf),
        }
    }
}

pub fn tar_sha256(filename: String) -> Result<(), Box<dyn Error>> {
    // `-` -> stdin
    let r = match filename.as_str() {
        "-" => Read1::S(stdin()),
        _ => Read1::F(File::open(filename)?),
    };
    let mut a = Archive::new(r);

    for i in a.entries()? {
        let mut f = i?;
        // only check normal file
        if f.header().entry_type() != EntryType::Regular {
            continue;
        }

        let p: String = f.path()?.to_string_lossy().into();
        // read buffer
        let mut b = [0; 2048];
        // hash
        let mut h = Sha256::new();

        loop {
            let l = f.read(&mut b)?;
            if 0 == l {
                break;
            }

            h.update(&b[0..l]);
        }
        // hex
        let h1 = h.finalize();
        let hh = base16ct::lower::encode_string(&h1);
        println!("{}  {}", hh, p);
    }
    Ok(())
}
