use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use sha2::{Digest, Sha256};
use tar::{Archive, EntryType};

pub fn tar_sha256(filename: String) -> Result<(), Box<dyn Error>> {
    let f = File::open(filename)?;
    let mut a = Archive::new(f);

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
