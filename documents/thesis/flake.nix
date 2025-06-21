{
  description = "A report built with Pandoc, with continious compilation.";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit system;};
        });
  in {
    formatter = forAllSystems ({pkgs}: pkgs.alejandra);

    packages = forAllSystems ({pkgs}: let
      fonts = pkgs.makeFontsConf { fontDirectories = [ pkgs.dejavu_fonts ]; };
      latexPkgs =  with pkgs; [
        pandoc
        haskellPackages.pandoc-crossref
        texlive.combined.scheme-full
        librsvg
      ];
      mk-pandoc-script = pkgs.writeShellApplication {
        name = "mk-pandoc-script";
        runtimeInputs = latexPkgs;
        text = ''
          pandoc \
            ./01-introduction/index.md \
            ./02-prerequisites/index.md \
            ./03-chain-of-signatures/index.md \
            ./04-pcdl/index.md \
            ./05-asdl/index.md \
            ./06-plonk/01-index.md \
            ./06-plonk/02-arithmetizer.md \
            ./06-plonk/03-trace.md \
            ./06-plonk/04-protocol.md \
            ./07-appendix/index.md \
            ./thesis.md \
            -H header.tex \
            --citeproc \
            --metadata date="$(date -d "@${toString self.lastModified}" -u "+%Y-%m-%d - %H:%M:%S %Z")" \
            --highlight-style gruvbox.theme \
            -o "$1/thesis.pdf"
        '';
      };
      mk-pandoc = pkgs.writeShellApplication {
        name = "mk-pandoc";
        runtimeInputs = latexPkgs;
        text = ''${pkgs.lib.getExe mk-pandoc-script} "."'';
      };
      mk-pandoc-loop = pkgs.writeShellApplication {
        name = "pandoc-compile-continuous";
        runtimeInputs = [mk-pandoc pkgs.fswatch];
        text = ''
          set +e
          mk-pandoc
          echo "Listening for file changes"
          fswatch --event Updated ./*.md | xargs -n 1 sh -c 'date "+%Y-%m-%d - %H:%M:%S %Z"; mk-pandoc'
        '';
      };
      spellcheck = pkgs.writeShellApplication {
        name = "spellcheck";
        runtimeInputs = [pkgs.nodePackages_latest.cspell];
        text = ''cspell "**/*.md"'';
      };
      spellcheck-watch = pkgs.writeShellApplication {
        name = "spellcheck";
        runtimeInputs = [pkgs.nodePackages_latest.cspell];
        text = ''watch --color cspell --color "**/*.md"'';
      };
      thesis = pkgs.stdenv.mkDerivation {
        name = "thesis";
        src = ./.;
        buildInputs = latexPkgs;
        phases = ["unpackPhase" "buildPhase"];
        buildPhase = ''
          export FONTCONFIG_FILE=${fonts}
          mkdir -p $out
          ${pkgs.lib.getExe mk-pandoc-script} "$out"
        '';
      };
      in {
        default = thesis;
        loop = mk-pandoc-loop;
        pandoc = mk-pandoc;
        spellcheck = spellcheck;
        spellcheck-watch = spellcheck-watch;
      });
  };
}
