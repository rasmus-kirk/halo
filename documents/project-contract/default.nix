{pkgs, self, ...}: let
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
      # Loop through each .md file in the folder
      for filename in ./*.md; do
          pandoc "$filename" \
            --metadata "date -d "@${toString self.lastModified}" -u '+%Y-%m-%d - %H:%M:%S %Z'" \
            -o "$1/''${filename%.md}.pdf"
      done
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
      echo "Listening for file changes"
      fswatch --event Updated ./*.md | xargs -n 1 sh -c 'date "+%Y-%m-%d - %H:%M:%S %Z"; mk-pandoc'
    '';
  };
  spellcheck = pkgs.writeShellApplication {
    name = "spellcheck";
    runtimeInputs = [pkgs.nodePackages_latest.cspell];
    text = ''cspell "*.md"'';
  };
  spellcheck-watch = pkgs.writeShellApplication {
    name = "spellcheck";
    runtimeInputs = [pkgs.nodePackages_latest.cspell];
    text = ''watch --color cspell --color "*.md"'';
  };
  contract = pkgs.stdenv.mkDerivation {
    name = "contract";
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
  default = contract;
  contract = contract;
  loop = mk-pandoc-loop;
  mk-pandoc = mk-pandoc;
  spellcheck = spellcheck;
  spellcheck-watch = spellcheck-watch;
}
