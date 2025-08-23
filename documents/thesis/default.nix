{pkgs, self, lib, resholve, writeTextFile, runtimeShell, stdenv, shellcheck}: let
  fonts = pkgs.makeFontsConf { fontDirectories = [ pkgs.dejavu_fonts ]; };
  latexPkgs =  with pkgs; [
    uutils-coreutils-noprefix
    pandoc
    haskellPackages.pandoc-crossref
    texlive.combined.scheme-full
    librsvg
    uutils-findutils
  ];
  # writeShellApplication with clean path
  writeShellApplication =
    { name
    , text
    , runtimeInputs ? [ ]
    , checkPhase ? null
    }:
    writeTextFile {
      inherit name;
      executable = true;
      destination = "/bin/${name}";
      text = ''
        #!${runtimeShell}
        set -o errexit
        set -o nounset
        set -o pipefail
      '' + lib.optionalString (runtimeInputs == [ ]) ''

        export PATH=""
      '' + ''
      '' + lib.optionalString (runtimeInputs != [ ]) ''

        export PATH="${lib.makeBinPath runtimeInputs}"
      '' + ''

        ${text}
      '';

      checkPhase =
        if checkPhase == null then ''
          runHook preCheck
          ${stdenv.shellDryRun} "$target"
          ${shellcheck}/bin/shellcheck "$target"
          runHook postCheck
        ''
        else checkPhase;

      meta.mainProgram = name;
    };
  mk-pandoc-script = writeShellApplication {
    name = "mk-pandoc-script";
    runtimeInputs = latexPkgs;
    text = ''
      shopt -s globstar nullglob

      if [ ! -d "$2" ]; then
        echo "Error: Out dir ($2) is not a directory" >&2
        exit 1
      fi

      name="$1"
      out="$2"
      in=()
      shift 2

      for arg in "$@"; do
        # If argument is a file, add it to the array
        if [ -f "$arg" ]; then
          in+=("$arg")
        # If argument is a directory, find all .md files recursively and add to array
        elif [ -d "$arg" ]; then
          while IFS= read -r file; do
            in+=("$file")
          done < <(find "$arg" -type f -name "*.md")
        else
          echo "Error: '$arg' is neither a file nor a directory." >&2
          exit 1
        fi
      done

      #sort
      mapfile -t in < <(for i in "''${in[@]}"; do echo "$i"; done | sort -n)

      pandoc \
        "''${in[@]}" \
        -H ${self}/documents/thesis/header.tex \
        --metadata-file ${self}/documents/thesis/metadata.yaml \
        --resource-path ${self}/documents/thesis \
        --citeproc \
        --metadata date="$(date -d "@${toString self.lastModified}" -u "+%Y-%m-%d - %H:%M:%S %Z")" \
        --highlight-style ${self}/documents/thesis/gruvbox.theme \
        -o "$out/$name"
    '';
  };
  mk-pandoc = writeShellApplication {
    name = "mk-pandoc";
    runtimeInputs = [ mk-pandoc-script ];
    text = ''
      if [ $# -eq 0 ]; then
        echo "Error: No arguments provided. Please provide at least one file or directory." >&2
        exit 1
      fi

      mk-pandoc-script "out.pdf" "." "$@"
    '';
  };
  mk-pandoc-loop = writeShellApplication {
    name = "pandoc-compile-continuous";
    runtimeInputs = [
      mk-pandoc
      pkgs.entr
      pkgs.uutils-findutils
      pkgs.bash
      pkgs.uutils-coreutils-noprefix
      pkgs.bc
    ];
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
  spellcheck = writeShellApplication {
    name = "spellcheck";
    runtimeInputs = [pkgs.nodePackages_latest.cspell];
    text = ''cspell "**/*.md"'';
  };
  spellcheck-watch = writeShellApplication {
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
      ${pkgs.lib.getExe mk-pandoc-script} "thesis.pdf" "$out" \
        ${self}/documents/thesis
        # ${self}/documents/thesis/01-introduction \
        # ${self}/documents/thesis/02-prerequisites \
        # ${self}/documents/thesis/03-chain-of-signatures \
        # ${self}/documents/thesis/04-pcdl \
        # ${self}/documents/thesis/05-asdl \
        # ${self}/documents/thesis/06-plonk \
        # ${self}/documents/thesis/07-appendix
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
