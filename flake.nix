{
  description = "spyglass devshell and package";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          name = "spyglass-devshell";

          packages = with pkgs; [
            go
            gopls
            gotools
            delve
            just
          ];
        };

        packages.spyglass = pkgs.buildGoModule {
          pname = "spyglass";
          version = "2026.06.25-a";

          src = self;

          vendorHash = "sha256-XmWORop+D+zY0hU2hb9CC2xwihQ/5aprh02Yj6zlyo8=";

          subPackages = [ "." ];
          ldflags = [ "-s" "-w" ];

          meta = with pkgs.lib; {
            description = "An extensible search tool, inspired by Raycast and Vicinae";
            license = licenses.mit;
            platforms = platforms.all;
          };
        };

        apps.spyglass = {
          type = "app";
          program = "${self.packages.${system}.spyglass}/bin/spyglass";
        };
      });
}
