{
  pkgs ? import <nixpkgs> {},
  fenix,
}: let
  getLibFolder = pkg: "${pkg}/lib";

  toolchain = with fenix.packages.${pkgs.system};
    combine [
      stable.toolchain
      targets.wasm32-unknown-unknown.stable.rust-std
    ];
in
  pkgs.stdenv.mkDerivation {
    name = "rust-website";

    nativeBuildInputs = with pkgs; [
      # LLVM & GCC
      gcc
      cmake
      gnumake
      pkg-config
      llvmPackages.llvm
      llvmPackages.clang
      llvmPackages.bintools

      # Hail the Nix
      nixd
      statix
      alejandra

      # Launch scripts
      just

      # Rust
      rustc
      cargo
      clippy
      trunk
      toolchain
      wasm-pack
      cargo-watch
      rust-analyzer
      wasm-bindgen-cli
    ];

    buildInputs = with pkgs; [
      openssl
    ];

    # Set Environment Variables
    RUST_BACKTRACE = 1;
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.gcc
      pkgs.libiconv
      pkgs.llvmPackages.llvm
    ];
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  }
