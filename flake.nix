{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/a7abebc31a8f60011277437e000eebcc01702b9f";
    rust-overlay.url = "github:oxalica/rust-overlay/47beae969336c05e892e1e4a9dbaac9593de34ab";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix/72db7ea069f055d5c7856aca091179a070201931"; 
    crane.url = "github:ipetkov/crane";
    movement.url = "github:movementlabsxyz/movement/aa1ffed1a113441a65662792d15682ad52406108";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, foundry, crane, movement, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) foundry.overlay ];
        };

        toolchain = p: (p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rustfmt" "clippy" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain(toolchain);

        frameworks = pkgs.darwin.apple_sdk.frameworks;

        # a function to handle the movement repository
        # NOTE: this is not a derivation.
        # We do not use derivations because we need to perform mutable operations on the directory; that's the simples path to embedding movement. 
        handleMovement = ''
          # Create .vendors directory if it doesn't exist
          mkdir -p .vendors

          # Handle Movement repository
          if [ -d ".vendors/movement" ]; then
            # Save target directory if it exists
            if [ -d ".vendors/movement/target" ]; then
              mv .vendors/movement/target /tmp/movement-target
            fi
            # Remove existing directory
            rm -rf .vendors/movement
          fi

          # Clone fresh copy
          git clone https://github.com/movementlabsxyz/movement.git .vendors/movement
          cd .vendors/movement
          git checkout ${movement.rev}

          # Restore target directory if it existed
          if [ -d "/tmp/movement-target" ]; then
            mv /tmp/movement-target target
          fi
          cd ../..
        '';

        # An LLVM build environment
        buildDependencies = with pkgs; [
          perl
          llvmPackages.bintools
          openssl
          openssl.dev
          libiconv 
          pkg-config
          libclang.lib
          libz
          clang
          pkg-config
          protobuf
          rustPlatform.bindgenHook
          lld
          coreutils
          gcc
          rust
          zlib
          pandoc
          postgresql
        ] ++ lib.optionals stdenv.isDarwin [
          fixDarwinDylibNames
        ];
        
        sysDependencies = with pkgs; [] 
        ++ lib.optionals stdenv.isDarwin [
          frameworks.Security
          frameworks.CoreServices
          frameworks.SystemConfiguration
          frameworks.AppKit
          libelf
        ] ++ lib.optionals stdenv.isLinux [
          udev
          systemd
          bzip2
          elfutils
          jemalloc
        ];

        testDependencies = with pkgs; [
          python311
          just
          foundry-bin
          process-compose
          jq
          docker
          solc
          grpcurl
          grpcui
        ];

        # Specific version of toolchain
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
    
      in {
        devShells = rec {
          default = docker-build;
          docker-build = pkgs.mkShell {
            ROCKSDB = pkgs.rocksdb;
            OPENSSL_DEV = pkgs.openssl.dev;

            hardeningDisable = ["fortify"];

            buildInputs = with pkgs; [
              # rust toolchain
              (toolchain pkgs)
            ] ++ sysDependencies ++ buildDependencies ++ testDependencies;

            LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib/";

            shellHook = ''
              #!/usr/bin/env ${pkgs.bash}

              # Export linker flags if on Darwin (macOS)
              if [[ "${pkgs.stdenv.hostPlatform.system}" =~ "darwin" ]]; then
                export MACOSX_DEPLOYMENT_TARGET=$(sw_vers -productVersion)
                export LDFLAGS="-L/opt/homebrew/opt/zlib/lib"
                export CPPFLAGS="-I/opt/homebrew/opt/zlib/include"
              fi

              # Add ./target/debug/* to PATH
              export PATH="$PATH:$(pwd)/target/debug"

              # Add ./target/release/* to PATH
              export PATH="$PATH:$(pwd)/target/release"

              ${handleMovement}

               # Copy over ./githooks/pre-commit to .git/hooks/pre-commit
              cp $(pwd)/.githooks/pre-commit $(pwd)/.git/hooks/pre-commit
              chmod +x $(pwd)/.git/hooks/pre-commit

              cat <<'EOF'
               MOVEMENT => MAPTOS
              EOF

              echo "Migrates Movement to Movement Aptos."
            '';
          };
        };
      }
    );
}
