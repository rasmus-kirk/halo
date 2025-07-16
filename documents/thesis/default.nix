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
      shopt -s globstar nullglob

      # Determine the correct date command
      if command -v gdate > /dev/null; then
        DATE_CMD="gdate"
      else
        DATE_CMD="date"
      fi

      if [ -z "''${1:-}" ]; then
        echo "Error: Missing first argument, the input directory or file." >&2
        exit 1
      elif [ -f "$1" ]; then
        IN="$1"
      elif [ -d "$1" ]; then
        IN=("$1"/**/*.md)
      else
        echo "Error: '$1' is neither a file nor a directory." >&2
        exit 1
      fi

      if [ -z "''${2:-}" ]; then
        echo "Error: Missing second argument, the output directory." >&2
        exit 1
      elif [ -d "$2" ]; then
        OUT="$2"
      else
        echo "Error: '$2' is not a directory." >&2
        exit 1
      fi

      pandoc \
        "''${IN[@]}" \
        -H ${self}/documents/thesis/header.tex \
        --metadata-file ${self}/documents/thesis/metadata.yaml \
        --resource-path ${self}/documents/thesis \
        --citeproc \
        --metadata date="$($DATE_CMD -d "@${toString self.lastModified}" -u "+%Y-%m-%d - %H:%M:%S %Z")" \
        --highlight-style ${self}/documents/thesis/gruvbox.theme \
        -o "$OUT/out.pdf"
    '';
  };
  mk-pandoc-loop = pkgs.writeShellApplication {
    name = "pandoc-compile-continuous";
    runtimeInputs = [mk-pandoc-script pkgs.entr];
    text = ''
      set +e
      if [ -z "''${1:-}" ]; then
        IN="."
      else
        IN="$1"
      fi

      # shellcheck disable=SC2016
      find . -type f -name "*.md" | entr -r sh -c '
        start=$(date +%s.%N)
        mk-pandoc-script "$0" .
        end=$(date +%s.%N)

        delta=$(echo "$end - $start" | bc)
        date=$(date "+%H:%M:%S")

        echo "$1: $date ($delta s)"
      ' "$IN" "$1"
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
      ${pkgs.lib.getExe mk-pandoc-script} "${self}/documents/thesis" "$out"
    '';
  };
in {
  thesis = thesis;
  default = thesis;
  loop = mk-pandoc-loop;
  spellcheck = spellcheck;
  spellcheck-watch = spellcheck-watch;
}
