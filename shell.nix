{ pkgs }:

with pkgs;

mkShell {
  buildInputs = [
    git
    hub

    rustc
    cargo
    rustfmt
    rls
  ];
}
