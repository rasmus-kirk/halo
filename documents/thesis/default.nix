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

      start_ns=$(date +%s%N)
      (
        sp='⣾⣽⣻⢿⡿⣟⣯⣷'
        while :; do
          for c in $(echo "$sp" | fold -w1); do
            end_ns=$(date +%s%N)
            elapsed_ns=$((end_ns - start_ns))
            elapsed_ms=$((elapsed_ns / 1000000))
            printf "\r\033[90m[$(date '+%Y-%m-%d %H:%M:%S')]\033[0m \033[34m%s\033[0m building \033[34m%dms\033[0m " "$c" "$elapsed_ms"
            sleep 0.1
          done
        done
      ) &
      SPINNER_PID=$!

      output=$(mk-pandoc 2>&1)
      exit_code=$?
      end_ns=$(date +%s%N)

      kill "$SPINNER_PID"
      wait "$SPINNER_PID" 2>/dev/null

      elapsed_ns=$((end_ns - start_ns))
      elapsed_ms=$((elapsed_ns / 1000000))
      timestamp=$(date '+%Y-%m-%d %H:%M:%S')
      printf "\r%50s\r" ""
      if [ $exit_code -eq 0 ]; then
        printf "\033[90m[%s]\033[0m \033[32m⣿\033[0m watching files \033[32m%dms\033[0m\n" "$timestamp" "$elapsed_ms"
      else
        printf "\033[90m[%s]\033[0m \033[31m⣿\033[0m watching files \033[31m%dms\033[0m\n" "$timestamp" "$elapsed_ms"
        [ -n "$output" ] && printf "%s\n" "$output"
      fi

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
              printf "\r%50s\r" ""
              timestamp=$(date '+%Y-%m-%d %H:%M:%S')
              printf "\033[90m[%s] ⣿\033[0m %s\n" "$timestamp" "$rel_file"
              
              start_ns=$(date +%s%N)
              (
                sp='⣾⣽⣻⢿⡿⣟⣯⣷'
                while :; do
                  for c in $(echo "$sp" | fold -w1); do
                    end_ns=$(date +%s%N)
                    elapsed_ns=$((end_ns - start_ns))
                    elapsed_ms=$((elapsed_ns / 1000000))
                    printf "\r\033[90m[$(date '+%Y-%m-%d %H:%M:%S')]\033[0m \033[34m%s\033[0m building \033[34m%dms\033[0m " "$c" "$elapsed_ms"
                    sleep 0.1
                  done
                done
              ) &
              SPINNER_PID=$!
              
              output=$(mk-pandoc 2>&1)
              exit_code=$?
              end_ns=$(date +%s%N)

              kill "$SPINNER_PID"
              wait "$SPINNER_PID" 2>/dev/null

              elapsed_ns=$((end_ns - start_ns))
              elapsed_ms=$((elapsed_ns / 1000000))
              timestamp=$(date '+%Y-%m-%d %H:%M:%S')
              printf "\r%50s\r" ""
              if [ $exit_code -eq 0 ]; then
                printf "\033[90m[%s]\033[0m \033[32m⣿\033[0m rebuilt \033[32m%dms\033[0m " "$timestamp" "$elapsed_ms"
              else
                printf "\033[90m[%s]\033[0m \033[31m⣿\033[0m failed \033[31m%dms\033[0m\n" "$timestamp" "$elapsed_ms"
                [ -n "$output" ] && printf "%s\n" "$output"
              fi

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
