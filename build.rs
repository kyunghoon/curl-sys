use anyhow::Result;
use std::process::Command;

fn go() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    Command::new("./buildconf")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
    Command::new("./configure")
        .arg(format!("--prefix={}", out_dir))
        .arg("--with-darwinssl").arg("--disable-ldap").arg("--disable-ldaps").arg("--disable-shared")
        .arg("--disable-verbose").arg("--disable-manual").arg("--disable-crypto-auth")
        .arg("--disable-unix-sockets").arg("--disable-idn").arg("--disable-dict").arg("--disable-ares")
        .arg("--disable-rtsp").arg("--disable-ipv6").arg("--disable-proxy").arg("--disable-versioned-symbols")
        .arg("--enable-hidden-symbols").arg("--without-gnutls").arg("--without-libidn").arg("--without-librtmp")
        .arg("--disable-dict").arg("--disable-file").arg("--disable-ftp").arg("--disable-ftps")
        .arg("--disable-gopher").arg("--disable-imap").arg("--disable-imaps").arg("--disable-pop3")
        .arg("--disable-pop3s").arg("--disable-smb").arg("--disable-smbs").arg("--disable-smtp")
        .arg("--disable-smtps").arg("--disable-telnet").arg("--disable-tftp").arg("--disable-debug")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
    Command::new("make")
        .arg("install")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
    Command::new("make")
        .arg("-j8")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;

    println!("cargo:INCLUDE={}/include", out_dir);
    println!("cargo:LIB={}/lib", out_dir);

    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={}", e);
    }
}
