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
      # Determine the correct date command
      if command -v gdate > /dev/null; then
        DATE_CMD="gdate"
      else
        DATE_CMD="date"
      fi

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
        --metadata date="$($DATE_CMD -d "@${toString self.lastModified}" -u "+%Y-%m-%d - %H:%M:%S %Z")" \
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
        output=$(mk-pandoc 2>&1)
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
          now=$(date +%s)
          if (( now - last_run >= debounce_seconds )); then
              rel_file="./$(echo "$file" | sed -E 's|.*thesis/||')"
              printf "\r\033[K"
              run_build "$rel_file" "$rel_file" "$rel_file"
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
