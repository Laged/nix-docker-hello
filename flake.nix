{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  inputs.treefmt-nix.url = "github:numtide/treefmt-nix";
  inputs.flake-root.url = "github:srid/flake-root";

  outputs =
    inputs @ { parts
    , ...
    }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [
        inputs.nci.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.flake-root.flakeModule
      ];
      perSystem =
        { config
        , pkgs
        , ...
        }:
        let
          # shorthand for accessing outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          inherit (config.nci) outputs;
        in
        {
          # declare projects
          nci.projects."crates" = {
            path = ./.;
            # export all crates (packages and devshell) in flake outputs
            # alternatively you can access the outputs and export them yourself
            export = true;
          };
          # configure crates
          nci.crates = {
            "greeter" = {
              # look at documentation for more options
            };
            "namegen" = {
              drvConfig = {
                mkDerivation.buildInputs = [ pkgs.hello pkgs.rust-analyzer ];
              };
              # look at documentation for more options
            };
          };
          # export the project devshell as the default devshell
          devShells.default = outputs."crates".devShell;
          # export the release package of the crate as default package
          packages.default = outputs."greeter".packages.release;
          # lint
          treefmt.config = {
            inherit (config.flake-root) projectRootFile;
            package = pkgs.treefmt;
            programs.rustfmt.enable = true;
            programs.deadnix.enable = true;
            programs.statix.enable = true;
            programs.nixpkgs-fmt.enable = true;
            programs.taplo.enable = true;
          };
        };
    };
}
