{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    rustup
    cargo-watch
    rustfmt
  ];
}

# Development Guide 
# https://matthewrhone.dev/nixos-vscode-environment
# Run nix-shell
