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
      lib = pkgs.lib;
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
        shellHook = ''
          # https://discourse.nixos.org/t/27196
          # (?) openssl will get dynamically linked, so we still need to point at it inside the shell
          # TODO: this env var doesn't actually seem to get set?
          export LD_LIBRARY_PATH="${lib.makeLibraryPath [ pkgs.openssl ]}"
        '';
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

          pname = "ff14-utils";
          version = "0.1";

        };

      # we have a single package with bin/ folder containing all the binaries
      # so we create an app for each binary telling nix which one to run
      apps.${system} =
        let
          binNames = [
            "bicolor-gem-items"
            "cosmic-weather"
            "cosmocredit-items"
            "everkeep-certs"
            "heliometry-items"
            "leve-compare"
            "list-recipes"
            "map-compare"
            "materia-prices"
            "orange-scrip-items"
            "orange-scrips"
            "purple-scrip-items"
            "purple-scrips"
            "recipe-compare"
            "shopping"
            "specific-recipe"
          ];
          pkg = self.packages.${system}.default;
        in
        (lib.listToAttrs (
          map (n: {
            name = n;
            value = {
              type = "app";
              program = "${pkg}/bin/${n}";
            };
          }) binNames
        ));

      formatter.${system} = nixpkgs.legacyPackages.${system}.nixfmt-rfc-style;
    };
}
