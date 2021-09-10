{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-21.05;
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];

      # Function to generate a set based on supported systems
      forAllSystems = f:
        nixpkgs.lib.genAttrs supportedSystems (system: f system);

      nixpkgsFor = forAllSystems (system: import nixpkgs {
        inherit system;
        overlays = [ ];
      });

    in {
      devShell = forAllSystems (system: import ./shell.nix {
        pkgs = nixpkgsFor.${system};
      });
    };
}
