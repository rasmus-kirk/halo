{
  description = "Flake for GH-pages";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    # Thesis
    thesis.url = "./thesis/report";
    thesis.inputs.nixpkgs.follows = "nixpkgs";

    # contract
    contract.url = "./project-contract";
    contract.inputs.nixpkgs.follows = "nixpkgs";

    website-builder.url = "github:rasmus-kirk/website-builder";
    website-builder.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    website-builder,
    contract,
    thesis,
    ...
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit system;};
        });
  in {
    packages = forAllSystems ({pkgs}: let
      website = website-builder.lib {
        pkgs = pkgs;
        src = ./.;
        timestamp = self.lastModified;
        headerTitle = "Halo Accumulation Scheme";
        includedDirs = [
          thesis.outputs.packages."${pkgs.system}".default
          contract.outputs.packages."${pkgs.system}".default
        ];
        standalonePages = [{
          title = "Investigating IVC with Accumulation Schemes";
          inputFile = ./README.md;
          outputFile = "index.html";
        }];
        navbar = [
          {
            title = "Home";
            location = "/";
          }
          {
            title = "Report";
            location = "/thesis/report.pdf";
          }
          {
            title = "Project Contract";
            location = "/contract/contract.pdf";
          }
          {
            title = "Github";
            location = "https://github.com/rasmus-kirk/halo";
          }
        ];
      };
    in {
      default = website.package;
      debug = website.loop;
    });

    formatter = forAllSystems ({pkgs}: pkgs.alejandra);
  };
}
