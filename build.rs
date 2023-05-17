use std::{
    env::var,
    error::Error,
    fs::{self, File},
    io::Write,
    process::Command,
};

use chrono::{Datelike, Utc};

//

fn main() -> Result<(), Box<dyn Error>> {
    let kernel = var("CARGO_PKG_NAME")?;
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
    //let arch = var("CARGO_CFG_TARGET_ARCH")?;
    //println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    let mut bootloader: Option<&'static str> = None;
    let mut set = |s| {
        if let Some(already) = bootloader {
            println!("cargo:warning=Bootloaders {s} and {already} are mutually exclusive");
            panic!();
        } else {
            bootloader = Some(s);
        }
    };
    #[cfg(feature = "limine")]
    set("limine");
    #[cfg(feature = "bootboot")]
    set("bootboot");
    #[cfg(feature = "multiboot1")]
    set("multiboot1");
    #[cfg(feature = "multiboot2")]
    set("multiboot2");

    if let Some(bootloader) = bootloader {
        let script = format!("src/boot/{bootloader}/link.ld");
        println!("cargo:rustc-link-arg-bin={kernel}=--script={script}");
        println!("cargo:rerun-if-changed={script}");
    } else {
        println!("cargo:warning=No bootloaders given");
        panic!();
    };

    // generate kernel font from the bitmap image
    // TODO: convert to proc-macro?

    let bmp_date = fs::metadata("./src/driver/video/font.bmp")
        .unwrap()
        .modified()
        .unwrap();
    let rs_date = fs::metadata("./src/driver/video/font.rs")
        .unwrap()
        .modified()
        .unwrap();
    // panic!("{bmp_date:?} {rs_date:?}");
    if bmp_date > rs_date {
        let mut generated_rs = File::options()
            .write(true)
            .truncate(true)
            .open("./src/video/font.rs")
            .unwrap();

        let bmp = image::open("./src/video/font.bmp").unwrap().to_luma8();
        assert_eq!(bmp.width(), 4096);
        assert_eq!(bmp.height(), 16);
        write!(
            generated_rs,
            "pub static FONT: [([u16; 16], bool); 256] = ["
        )
        .unwrap();

        for i in 0..=255_u8 {
            let mut byte = ([0u16; 16], false);

            bmp.chunks(16)
                .skip(i as usize)
                .step_by(256)
                .enumerate()
                .for_each(|(i, s)| {
                    s.iter().enumerate().for_each(|(j, b)| {
                        if *b != 255 {
                            byte.0[i] |= 1 << j
                        }
                    })
                });

            // set the flag if the character is 16 wide instead of 8 wide
            byte.1 = !byte.0.iter().all(|c| *c < 0x100);

            write!(generated_rs, "\n\t{byte:?},").unwrap();
        }

        write!(generated_rs, "\n];").unwrap();
    }

    // TODO: convert to proc-macro?
    let mut generated_date = File::options()
        .write(true)
        .truncate(true)
        .open("./src/driver/rtc.year")
        .unwrap();
    write!(generated_date, "{}", Utc::now().date_naive().year()).unwrap();

    /* println!(
        "cargo:rustc-env=HYPERION_RTC_YEAR={}",
        Utc::now().date_naive().year()
    ); */

    println!(
        "cargo:rustc-env=HYPERION_BUILD_TIME={}",
        Utc::now().naive_local().format("%Y-%m-%d %H:%M:%S")
    );

    let rev = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap()
        .stdout;
    println!(
        "cargo:rustc-env=HYPERION_BUILD_REV={}",
        std::str::from_utf8(&rev).unwrap()
    );

    Ok(())
}
