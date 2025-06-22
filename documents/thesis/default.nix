{pkgs, self}: let
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
      # mk-pandoc
      echo "Listening for file changes"

      last_run=0
      debounce_seconds=1

      fswatch -r . --event Updated \
        --exclude='.*\.aux$' \
        --exclude='.*\.log$' \
        --exclude='.*\.git/.*' \
        --exclude='.*result$' \
        --exclude='.*\.pdf$' \
        | grep -E --line-buffered '(\.md|\.tex|\.bib)$' \
        | while read -r file; do
          echo "test: $file"
          now=$(date +%s)
          if (( now - last_run >= debounce_seconds )); then
              echo "$(date '+%Y-%m-%d - %H:%M:%S %Z') - Rebuilding due to: $file"
              # mk-pandoc
              last_run=$now
          fi
      done
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
  thesis = thesis;
  default = thesis;
  loop = mk-pandoc-loop;
  pandoc = mk-pandoc;
  spellcheck = spellcheck;
  spellcheck-watch = spellcheck-watch;
}
