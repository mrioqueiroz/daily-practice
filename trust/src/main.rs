use std::io;

fn main() -> io::Result<()> {
    // Create a new interface (user space network)
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = vec![0u8; 1504];
    loop {
        let mut nbytes = nic.recv(&mut buf[..])?;
        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
    Ok(())
}
