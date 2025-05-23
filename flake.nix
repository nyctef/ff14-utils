{
  description = "minimal devshell flake for rust";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      crane,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      fenixPkgs = fenix.packages.${system}.default;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # rust toolchain
          fenix.packages.${system}.complete.toolchain
          # required by dependencies
          pkg-config
          openssl
        ];
      };

      packages.${system}.default =

        let
          lib = pkgs.lib;
          craneLib = crane.mkLib pkgs;
          c = craneLib.overrideToolchain fenix.packages.${system}.minimal.toolchain;
        in

        c.buildPackage {
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              # by default, only include .rs and .toml files to avoid unnecessary rebuilds
              (craneLib.fileset.commonCargoSources ./.)
              # also keep .json and .html files since we include_str! some of those
              (lib.fileset.fileFilter (file: file.hasExt "json") ./.)
              (lib.fileset.fileFilter (file: file.hasExt "html") ./.)
            ];
          };

          buildInputs = with pkgs; [
            pkg-config
            openssl

          ];

        };

      formatter.${system} = nixpkgs.legacyPackages.${system}.nixfmt-rfc-style;
    };
}
