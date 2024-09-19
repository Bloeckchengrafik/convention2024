{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, unstable }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        unstablePkgs = import unstable {
          inherit system;
        };

        libraries = with pkgs; [
          glib
          openssl_3
          vulkan-headers
          vulkan-loader
          vulkan-validation-layers
	  systemd
	  alsa-lib
	  wayland
          wayland-protocols
          libxkbcommon
	] ++ [unstablePkgs.onnxruntime];

        packages = with pkgs; [
          fontconfig
	  libxkbcommon
          glib
          libxkbcommon
          openssl_3
          pkg-config
          vulkan-tools
          wayland-scanner
          xorg.libxcb
	  systemd
	  vulkan-validation-layers
	  alsa-lib
	  nodejs_22
	  nodePackages.pnpm
	  platformio-core
          (pkgs.rust-bin.stable.latest.default.override
            { extensions = [ "rust-src" ]; })
        ] ++ [unstablePkgs.onnxruntime];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages;

          shellHook =
            ''
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:${pkgs.stdenv.cc.cc.lib}:${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
            '';
        };
      });
}
