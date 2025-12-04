let
pkgs = import <nixpkgs> {};

libraries = with pkgs;[
];

packages = with pkgs; [
    cargo
    rustc
    #rustup

    rust-analyzer
    rustfmt

    pandoc
];

in
pkgs.mkShell {
    buildInputs = packages;

    shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    '';
}
