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
        # ./01-introduction/*.md \
        # ./02-prerequisites/*.md \
        # ./03-chain-of-signatures/*.md \
        # ./04-pcdl/*.md \
        # ./05-asdl/*.md \
      pandoc \
        ./06-plonk/*.md \
        ./07-appendix/*.md \
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

      # Definitions ---------------------------------------

      FIRST_BUILD_PID=""

      # Debounce settings and progress characters
      debounce_ms=2500
      dbsp='▁▂▃▄▅▆▇█'
      N=8
      get_debounce_char() {
        i=0
        for c in $(echo "$dbsp" | fold -w1); do
          if (( i == $1 )) then
            sc=$c
            break
          fi
          i=$((i+1))
        done
      }

      # Get the current timestamp in milliseconds
      now_ms() {
        echo $(( $(date +%s%N) / 1000000 ))
      }

      # Get the current timestamp in a formatted string
      timestamp=""
      update_timestamp() {
        timestamp=$(printf '\033[90m[%s]\033[0m' "$(date '+%Y-%m-%d %H:%M:%S')")
      }

      # Restory tty settings
      restore_stty() {
        # Try Linux-style first
        if stty -F /dev/tty sane 2>/dev/null; then
          return
        fi

        # Fallback to macOS-style
        if stty sane < /dev/tty 2>/dev/null; then
          return
        fi

        # Final fallback (TTY not available?)
        echo "Warning: could not restore terminal settings" >&2
      }

      # Kill process if it exists
      kill_if_exist() {
        if [ -n "$1" ] && kill -0 "$1" 2>/dev/null; then
          kill "$1" 2>/dev/null
          wait "$1" 2>/dev/null
        fi
      }

      # Cleanup function
      killed=0
      cleanup() {
        # Guard against recursion
        if [ $killed -eq 1 ]; then
          return
        fi

        # Kill the first build process if it exists
        kill_if_exist "$FIRST_BUILD_PID"

        # Hack, wait for builder spinner to die
        sleep 0.4

        # Print shutdown message
        update_timestamp
        printf "\r\033[K"
        printf "%s \033[90m»\033[0m watcher has shut down\n" "$timestamp"

        # Restore terminal settings
        restore_stty

        # Set flag and exit
        killed=1
        kill 0
        exit 0
      }

      # Run the spinner then mk-pandoc and print the result
      sp='⣾⣽⣻⢿⡿⣟⣯⣷'
      run_build() {
        # Start the spinner in the background
        start_ns=$(date +%s%N)
        (
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

        # Print the result
        elapsed_ns=$((end_ns - start_ns))
        elapsed_ms=$((elapsed_ns / 1000000))
        update_timestamp
        printf "\r\033[K"
        if [ $exit_code -ne 130 ]; then
          if [ $exit_code -eq 0 ]; then
            printf "%s \033[32m✓\033[0m %s \033[32m%dms\033[0m\n" "$timestamp" "$2" "$elapsed_ms"
          else
            printf "%s \033[31m✕\033[0m %s \033[31m%dms\033[0m\n" "$timestamp" "$3" "$elapsed_ms"
            [ -n "$output" ] && printf "%s\n" "$output"
          fi
        fi
        printf "%s \033[90m»\033[0m watching files" "$timestamp"
      }

      # Process -------------------------------------------

      # Set up trap
      trap cleanup INT TERM

      # Disable input echo and canonical mode (no line buffering)
      stty -echo -icanon time 0 min 0

      # Run the initial build as a background process
      run_build "first build" "first build success" "first build failed" &
      FIRST_BUILD_PID=$!

      # Start watcher
      while read -r file; do
        # Guard initial build to finish first
        wait "$FIRST_BUILD_PID" 2>/dev/null || true
        
        # Debounce flush other changes
        num_changes=1
        lastread_ms=$(now_ms)
        while (( $(now_ms) - lastread_ms < debounce_ms )) do
          # Calculate debounce progress
          wait_ms=$((debounce_ms - $(now_ms) + lastread_ms))
          if (( wait_ms <= 0 )); then
            wait_ms=0
          fi

          # Print debounce spinner
          sc='?'
          get_debounce_char $(((wait_ms * N) / debounce_ms))
          update_timestamp
          printf "\r%s \033[90m%s\033[0m watching files " "$timestamp" "$sc"
          
          # Instead of sleep 0.1, we read until timeout 0.1
          while read -t 0.1 -r maybe_new; do
            num_changes=$((num_changes + 1))
            file="$maybe_new [$num_changes changes]"
            lastread_ms=$(now_ms)
          done
        done

        # Rebuild
        rel_file="./$(echo "$file" | sed -E 's|.*thesis/||')"
        printf "\r\033[K"
        run_build "$rel_file" "$rel_file" "$rel_file"
      done < <(
        fswatch -r . --event Updated \
          --exclude='.*\.aux$' \
          --exclude='.*\.log$' \
          --exclude='.*\.git/.*' \
          --exclude='.*result$' \
          --exclude='.*\.pdf$' \
          | grep -E --line-buffered '(\.md|\.tex|\.bib)$'
      )
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
