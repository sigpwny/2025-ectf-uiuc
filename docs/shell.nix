{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.pandoc
    pkgs.nodejs
    pkgs.tectonic
    pkgs.haskellPackages.pandoc-crossref
    pkgs.mermaid-filter
    pkgs.inter
    pkgs.texliveFull
    pkgs.librsvg
  ];
}
