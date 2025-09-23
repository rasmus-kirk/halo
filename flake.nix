{
  description = "Flake for GH-pages";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    website-builder.url = "github:rasmus-kirk/website-builder";
    website-builder.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    website-builder,
    ...
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit system;};
        });
    contractPkgsF = pkgs: pkgs.callPackage ./documents/project-contract { self = self; };
    thesisPkgsF = pkgs: pkgs.callPackage ./documents/thesis { self = self; };
    slidesPkgsF = pkgs: pkgs.callPackage ./documents/slides/rasmus { self = self; };
    cratesF = pkgs: pkgs.callPackage ./crates { self = self; rust-overlay = rust-overlay; };
    websiteF = pkgs: website-builder.lib {
      pkgs = pkgs;
      src = ./.;
      timestamp = self.lastModified;
      headerTitle = "Halo Accumulation Scheme";
      includedDirs = [
        (thesisPkgsF pkgs).default
        (slidesPkgsF pkgs).default
        (contractPkgsF pkgs).default
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
          location = "/thesis/thesis.pdf";
        }
        {
          title = "Project Contract";
          location = "/contract/contract.pdf";
        }
        {
          title = "Slides";
          location = "/slides/slides.pdf";
        }
        {
          title = "Github";
          location = "https://github.com/rasmus-kirk/halo";
        }
      ];
    };
  in {
    packages = forAllSystems ({pkgs}: rec {
      website = websiteF pkgs;
      contract = contractPkgsF pkgs;
      thesis = thesisPkgsF pkgs;
      slides = slidesPkgsF pkgs;
      crates = (cratesF pkgs).packages;
      default = website.package;
    });

    devShells = forAllSystems ({pkgs}: {
      default = (cratesF pkgs).devShells.default;
    });

    formatter = forAllSystems ({pkgs}: pkgs.alejandra);
  };
}
