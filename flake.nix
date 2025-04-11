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
          craneLib = crane.mkLib pkgs;
          c = craneLib.overrideToolchain fenix.packages.${system}.minimal.toolchain;
        in

        c.buildPackage {
          # cleanCargoSource: filter out any files not directly related to the build, so we rebuild less often
          src = craneLib.cleanCargoSource ./.;
          buildInputs = with pkgs; [
            pkg-config
            openssl

          ];

        };

      formatter.${system} = nixpkgs.legacyPackages.${system}.nixfmt-rfc-style;
    };
}
