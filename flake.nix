{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = 
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , ... } 
    @inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustTools = with pkgs; rust-bin.stable.latest.default.override {
          extensions = ["rust-analyzer" "rust-src" "rust-std" "rust-docs"];
        };
      in 
        { 
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustTools 
              opam
              pkg-config
              libffi
              gmp
              clang
              llvmPackages.libclang
              glibc.dev
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            BINDGEN_EXTRA_CLANG_ARGS = 
              "-I${pkgs.glibc.dev}/include " +
              "-I${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include";

          };
        }
    );
}
