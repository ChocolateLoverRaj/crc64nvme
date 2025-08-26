use std::{
    io::{self, Read, Write, stdin, stdout},
    num::NonZero,
};

use crc64fast_nvme::Digest;

fn main() -> io::Result<()> {
    let mut b = [Default::default(); 0x1000];
    let mut c = Digest::new();
    let mut stdin = stdin().lock();
    while let Some(len) = NonZero::new(stdin.read(&mut b)?) {
        c.write(&b[..len.get()]);
    }
    stdout().write_all(&c.sum64().to_be_bytes())?;
    Ok(())
}
