with import <nixpkgs> {};

let
    mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
    channel = pkgs.rustChannelOf {
        date = "2023-09-01";
        channel = "nightly";
    };
    rust = (channel.rust.override {
        targets = [ "x86_64-unknown-linux-musl" ];
        extensions = [ "rust-src" ];
    });
in pkgs.mkShell {
    name = "rust-env";
    buildInputs = [
        rust
        pkgs.pkg-config
        pkgs.libusb1
    ];

    PKG_CONFIG_ALLOW_CROSS = true;
    PKG_CONFIG_ALL_STATIC = true;
    LIBZ_SYS_STATIC = 1;
}
