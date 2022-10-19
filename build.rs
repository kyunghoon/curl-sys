use anyhow::Result;
use std::{process::Command, env::var};

macro_rules! check {
    ($out: expr) => {
        let out = $out;
        if !out.status.success() {
            panic!("{}", std::str::from_utf8(&out.stderr)?);
        }
    };
}

fn go() -> Result<()> {
    let out_dir = var("OUT_DIR").unwrap();

    check!(Command::new("./buildconf")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?);

    match std::env::var("TARGET") {
        Ok(target) if target == "aarch64-linux-android" => {
            match std::env::var("HOST") {
                Ok(host) if host == "x86_64-apple-darwin" => {
                    //PKG_CONFIG_PATH=$PWD/../output_openssl_android/usr/local/lib/pkgconfig LIBS="-ldl"
                    check!(Command::new("./configure")
                        .env("PKG_CONFIG_PATH", format!("{}/pkgconfig", var("DEP_OPENSSL_LIB").unwrap()))
                        .env("LIBS", "-dl")
                        .arg(format!("--prefix={}", out_dir))
                        .arg(format!("--host={}", host))
                        .arg(format!("--target={}", target))
                        .arg(format!("--with-ssl=\"{}/..\"", var("DEP_OPENSSL_INCLUDE").unwrap()))
                        .arg("--disable-shared").arg("--disable-verbose").arg("--disable-manual").arg("--disable-crypto-auth")
                        .arg("--disable-unix-sockets").arg("--disable-dict").arg("--disable-ares").arg("--disable-rtsp")
                        .arg("--disable-ipv6").arg("--disable-proxy").arg("--disable-versioned-symbols").arg("--enable-hidden-symbols")
                        .arg("--without-gnutls").arg("--without-libidn").arg("--without-librtmp").arg("--with-zlib")
                        .arg("--disable-dict").arg("--disable-file").arg("--disable-ftp").arg("--disable-ftps")
                        .arg("--disable-gopher").arg("--disable-imap").arg("--disable-imaps").arg("--disable-pop3")
                        .arg("--disable-pop3s").arg("--disable-smb").arg("--disable-smbs").arg("--disable-smtp")
                        .arg("--disable-smtps").arg("--disable-telnet").arg("--disable-tftp").arg("--disable-debug")
                        .arg("--enable-optimize")
                        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?);
                }
                x => panic!("{:?} not yet supported", x),
            }
        },
        _ => {
            check!(Command::new("./configure")
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
                .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?);
        }
    };

    check!(Command::new("make")
        .arg("-j8")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?);
    check!(Command::new("make")
        .arg("install")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?);

    println!("cargo:INCLUDE={}/include", out_dir);
    println!("cargo:LIB={}/lib", out_dir);

    Ok(())
}
fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={}", e);
    }
}
