{ pkgs ? import <nixpkgs> { }
, fenix,
}:
let
  lib = pkgs.lib;
  getLibFolder = pkg: "${pkg}/lib";

  toolchain = with fenix.packages.${pkgs.system};
    combine [
      stable.toolchain
      targets.wasm32-unknown-unknown.stable.rust-std
    ];


  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "rust-website";
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = with pkgs; [
    # LLVM & GCC
    gcc
    cmake
    gnumake
    pkg-config
    llvmPackages.llvm
    llvmPackages.clang
    llvmPackages.bintools

    # Tailwindcss
    tailwindcss

    # Rust
    rustc
    cargo
    trunk
    clippy
    libiconv
    toolchain
    wasm-pack
    pkg-config
    rust-analyzer
    wasm-bindgen-cli
  ];

  buildInputs = with pkgs; [
    wasm-bindgen-cli
  ];

  buildPhase = ''
    # Build the css
    tailwindcss -i ./style/input.css -o ./style/output.css

    # Wasm-bindgen nigga can't build without storing cache at HOME
    export HOME="$(pwd)/home"
    mkdir -p $HOME

    # Create the dist folder
    cargo build --release

    # Build wasm webiste
    trunk build --release --public-url=/
  '';

  installPhase = ''
    # Create out directory
    mkdir -p $out/www

    # Move all finished content
    mv ./dist/* $out/www
  '';

  # If you wanna get thorny
  # RUST_BACKTRACE = 1;
  NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
     pkgs.gcc
     pkgs.libiconv
     pkgs.llvmPackages.llvm
  ];
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

  meta = with lib; {
    homepage = manifest.package.homepage;
    description = "Website of Rust Uzbekistan community";
    license = with lib.licenses; [ gpl3Only ];

    platforms = with platforms; linux ++ darwin;

    maintainers = [
      {
        name = "Sokhibjon Orzikulov";
        email = "sakhib@orzklv.uz";
        handle = "orzklv";
        github = "orzklv";
        githubId = 54666588;
        keys = [
          {
            fingerprint = "00D2 7BC6 8707 0683 FBB9  137C 3C35 D3AF 0DA1 D6A8";
          }
        ];
      }
    ];
  };
}
