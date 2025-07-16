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

      if [ $# -eq 0 ]; then
        echo "Error: No arguments provided. Please provide at least one file or directory." >&2
        exit 1
      fi

      if [ -z "''${1:-}" ]; then
        echo "Error: Missing first argument, the output directory." >&2
        exit 1
      elif [ -d "$1" ]; then
        OUT="$1"
      else
        echo "Error: '$1' is not a directory." >&2
        exit 1
      fi

      shift
      IN=()
      for arg in "$@"; do
        # If argument is a file, add it to the array
        if [ -f "$arg" ]; then
          IN+=("$arg")
        # If argument is a directory, find all .md files recursively and add to array
        elif [ -d "$arg" ]; then
          while IFS= read -r file; do
            IN+=("$file")
          done < <(find "$arg" -type f -name "*.md")
        else
          echo "Error: '$arg' is neither a file nor a directory." >&2
          exit 1
        fi
      done


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
  mk-pandoc = pkgs.writeShellApplication {
    name = "mk-pandoc";
    runtimeInputs = [ mk-pandoc-script ];
    text = ''
      mk-pandoc-script "." "$@"
    '';
  };
  mk-pandoc-loop = pkgs.writeShellApplication {
    name = "pandoc-compile-continuous";
    runtimeInputs = [mk-pandoc pkgs.entr];
    text = ''
      set +e
      if [ -z "''${1:-}" ]; then
        set -- "."
      fi

      # shellcheck disable=SC2016
      find . -type f -name "*.md" | entr -r sh -c '
        start=$(date +%s.%N)
        mk-pandoc "$@"
        end=$(date +%s.%N)

        rel_path=$(realpath --relative-to="." "$0")
        date=$(date "+%H:%M:%S")
        delta=$(echo "$end - $start" | bc)

        echo "./$rel_path: $date ($delta s)"
      ' /_ "$@"
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
  pandoc = mk-pandoc-script;
  spellcheck = spellcheck;
  spellcheck-watch = spellcheck-watch;
}
