use anyhow::Result;
use std::{process::Command, env::var};

macro_rules! check {
    ($out: ident) => {
        let out = $out;
        if !out.status.success() {
            panic!("{}", std::str::from_utf8(&out.stderr)?);
        }
    };
}

fn go() -> Result<()> {
    let out_dir = var("OUT_DIR").unwrap();

    let out = Command::new("./buildconf")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
    check!(out);

    match std::env::var("TARGET") {
        Ok(target) if target == "aarch64-linux-android" => {
            match std::env::var("HOST") {
                Ok(host) if host == "x86_64-apple-darwin" || host == "aarch64-apple-darwin" => {
                    let ndk_root = var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME is undefined");
                    let toolchain = format!("{}/toolchains/{}-4.9/prebuilt/darwin-x86_64/bin", ndk_root, target);
                    let android_sysroot = format!("{}/platforms/android-24/arch-arm64", ndk_root);

                    let path = format!("{}:{}", toolchain.as_str(), var("PATH").unwrap());

                    let target_host = "aarch64-linux-android";

                    //PKG_CONFIG_PATH=$PWD/../output_openssl_android/usr/local/lib/pkgconfig LIBS="-ldl"
                    let out = Command::new("./configure")
                        .env("PATH", path)

                        .env("CPPFLAGS", format!("--sysroot={} -I{}", android_sysroot, var("DEP_OPENSSL_INCLUDE").unwrap()))
                        .env("CFLAGS", format!("--sysroot={}", android_sysroot))
                        .env("CXXFLAGS", format!("--sysroot={}", android_sysroot))
                        .env("LDFLAGS", format!("-L{}", var("DEP_OPENSSL_LIB").unwrap()))

                        .env("TARGET_HOST", target_host)
                        .env("ANDROID_ARCH", "arm64-v8a")

                        .env("AR", format!("{}/{}-ar", toolchain, target_host))
                        .env("AS", format!("{}/{}-as", toolchain, target_host))
                        .env("CC", format!("{}/{}-gcc", toolchain, target_host))
                        .env("CXX", format!("{}/{}-g++", toolchain, target_host))
                        .env("LD", format!("{}/{}-ld", toolchain, target_host))
                        .env("RANLIB", format!("{}/{}-ranlib", toolchain, target_host))
                        .env("STRIP", format!("{}/{}-strip", toolchain, target_host))

                        .env("PKG_CONFIG_PATH", format!("{}/pkgconfig", var("DEP_OPENSSL_LIB").unwrap()))
                        .env("LIBS", "-dl")
                        .arg(format!("--prefix={}", out_dir))
                        .arg(format!("--host={}", target_host))
                        .arg(format!("--target={}", target_host))
                        .arg(format!("--with-ssl=\"{}/..\"", var("DEP_OPENSSL_INCLUDE").unwrap()))
                        .arg("--disable-shared").arg("--disable-verbose").arg("--disable-manual").arg("--disable-crypto-auth")
                        .arg("--disable-unix-sockets").arg("--disable-dict").arg("--disable-ares").arg("--disable-rtsp")
                        .arg("--disable-ipv6").arg("--disable-proxy").arg("--disable-versioned-symbols").arg("--enable-hidden-symbols")
                        .arg("--without-gnutls").arg("--without-libidn").arg("--without-librtmp").arg("--with-zlib")
                        .arg("--disable-dict").arg("--disable-file").arg("--disable-ftp").arg("--disable-ftps")
                        .arg("--disable-gopher").arg("--disable-imap").arg("--disable-imaps").arg("--disable-pop3")
                        .arg("--disable-pop3s").arg("--disable-smb").arg("--disable-smbs").arg("--disable-smtp")
                        .arg("--disable-smtps").arg("--disable-telnet").arg("--disable-tftp")
                        //.arg(if var("PROFILE").unwrap() == "debug" { "--enable-debug" } else { "--disable-debug" })
                        .arg("--disable-debug")
                        .arg("--enable-optimize")
                        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
                        check!(out);
                }
                x => panic!("{:?} not yet supported", x),
            }
        },
        _ => {
            let host = std::env::var("HOST").unwrap();
            let mut cmd = match host.as_str() {
                h if h == "x86_64-apple-darwin" => Command::new("./configure"),
                h if h == "aarch64-apple-darwin" => Command::new("arch"),
                _ => panic!("unsupported host"),
            };

            let out = if host != "aarch64-apple-darwin" { &mut cmd } else {
                cmd.arg("-x86_64").arg("./configure")
            }.arg(format!("--prefix={}", out_dir))
                .arg("--with-darwinssl").arg("--disable-ldap").arg("--disable-ldaps").arg("--disable-shared")
                .arg("--disable-verbose").arg("--disable-manual").arg("--disable-crypto-auth")
                .arg("--disable-unix-sockets").arg("--disable-idn").arg("--disable-dict").arg("--disable-ares")
                .arg("--disable-rtsp").arg("--disable-ipv6").arg("--disable-proxy").arg("--disable-versioned-symbols")
                .arg("--enable-hidden-symbols").arg("--without-gnutls").arg("--without-libidn").arg("--without-librtmp")
                .arg("--disable-dict").arg("--disable-file").arg("--disable-ftp").arg("--disable-ftps")
                .arg("--disable-gopher").arg("--disable-imap").arg("--disable-imaps").arg("--disable-pop3")
                .arg("--disable-pop3s").arg("--disable-smb").arg("--disable-smbs").arg("--disable-smtp")
                .arg("--disable-smtps").arg("--disable-telnet").arg("--disable-tftp")
                //.arg(if var("PROFILE").unwrap() == "debug" { "--enable-debug" } else { "--disable-debug" })
                .arg("--disable-debug")
                .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
            check!(out);
        }
    };

    let out = Command::new("make")
        .arg("-j8")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
    check!(out);
    let out = Command::new("make")
        .arg("install")
        .current_dir(format!("{}/curl", env!("CARGO_MANIFEST_DIR"))).output()?;
    check!(out);

    println!("cargo:INCLUDE={}/include", out_dir);
    println!("cargo:LIB={}/lib", out_dir);

    Ok(())
}
fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={}", e);
    }
}
