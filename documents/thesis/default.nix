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
    runtimeInputs = [mk-pandoc-script pkgs.fswatch];
    text = ''
      set +e
      if [ -z "''${1:-}" ]; then
        DIR="."
      else
        DIR="$1"
      fi

      SPINNER_PID=""

      # This function updates the global variable `timestamp` string
      timestamp=""
      update_timestamp() {
        timestamp=$(printf '\033[90m[%s]\033[0m' "$(date '+%Y-%m-%d %H:%M:%S')")
      }

      # Trap cleanup for graceful exit and terminal restore
      killed=0
      cleanup() {
        if [ "x$SPINNER_PID" != "x" ] && kill -0 "$SPINNER_PID" 2>/dev/null; then
          kill "$SPINNER_PID" 2>/dev/null
          wait "$SPINNER_PID" 2>/dev/null
        fi

        if [ $killed -eq 1 ]; then
          return
        fi
        update_timestamp
        printf "\r\033[K"
        printf "%s shutting down watcher\n" "$timestamp"
        stty sane
        killed=1
        kill 0
        exit 0
      }
      trap cleanup INT TERM

      # Disable input echo and canonical mode (no line buffering)
      stty -echo -icanon time 0 min 0

      # This function runs the spinner and then calls mk-pandoc and prints the result
      run_build() {
        # Start the spinner in the background
        start_ns=$(date +%s%N)
        (
          sp='⣾⣽⣻⢿⡿⣟⣯⣷'
          while :; do
            for c in $(echo "$sp" | fold -w1); do
              end_ns=$(date +%s%N)
              elapsed_ns=$((end_ns - start_ns))
              elapsed_ms=$((elapsed_ns / 1000000))
              update_timestamp
              printf "\r%s \033[34m%s\033[0m %s \033[34m%dms\033[0m " "$timestamp" "$c" "$1" "$elapsed_ms"
              sleep 0.1
            done
          done
        ) &
        SPINNER_PID=$!

        # Run pandoc and capture its output
        output=$(mk-pandoc-script "$DIR" . 2>&1)
        exit_code=$?
        end_ns=$(date +%s%N)

        # Kill the spinner and wait for it to finish
        kill "$SPINNER_PID"
        wait "$SPINNER_PID" 2>/dev/null

        # Calculate elapsed time
        elapsed_ns=$((end_ns - start_ns))
        elapsed_ms=$((elapsed_ns / 1000000))
        update_timestamp

        # Clear the spinner line
        printf "\r\033[K"

        # Print the result
        if [ $exit_code -eq 0 ]; then
          printf "%s \033[32m✓\033[0m %s \033[32m%dms\033[0m\n" "$timestamp" "$2" "$elapsed_ms"
        else
          printf "%s \033[31m✕\033[0m %s \033[31m%dms\033[0m\n" "$timestamp" "$3" "$elapsed_ms"
          [ -n "$output" ] && printf "%s\n" "$output"
        fi

        # Print the watching message
        printf "%s \033[90m»\033[0m watching files" "$timestamp"
      }

      run_build "building" "built" "build failed"

      fswatch -r . --event Updated \
        --exclude='.*\.aux$' \
        --exclude='.*\.log$' \
        --exclude='.*\.git/.*' \
        --exclude='.*result$' \
        --exclude='.*\.pdf$' \
        | grep -E --line-buffered '(\.md|\.tex|\.bib)$' \
        | while read -r file; do
            num_files=0
            # Flush all pending lines to get the latest path
            while read -t 0.1 -r maybe_new; do
              num_files=$((num_files + 1))
              file="$maybe_new and $num_files other"
            done

            rel_file="./$(echo "$file" | sed -E 's|.*thesis/||')"
            printf "\r\033[K"
            run_build "$rel_file" "$rel_file" "$rel_file"
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
