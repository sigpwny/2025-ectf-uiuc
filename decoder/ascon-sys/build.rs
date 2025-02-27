use std::path::Path;

fn main() {
    let ascon_dir =
        Path::new("vendor/ascon-c/crypto_aead/ascon128v12/protected_bi32_armv6_leveled/");
    let sources = [
        "aead.c",
        "constants.c",
        "crypto_aead_shared.c",
        "crypto_aead.c",
        "interleave.c",
        "permutations.c",
        "printstate.c",
        "shares.c",
    ];

    cc::Build::new()
        .files(sources.map(|p| ascon_dir.join(p)))
        .include(ascon_dir)
        .include("include")
        .opt_level_str("s")
        .define("NUM_SHARES_KEY", "4")
        .compile("ascon");

    println!("cargo:rerun-if-changed=vendor/ascon-c")
}
