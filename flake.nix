{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/a7abebc31a8f60011277437e000eebcc01702b9f";
    rust-overlay.url = "github:oxalica/rust-overlay/47beae969336c05e892e1e4a9dbaac9593de34ab";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix/36a5c5e448b4cdc85813e3b8205a8846a428d528"; 
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

        # Create a proper Nix derivation for the movement repository
        movementRepo = pkgs.stdenv.mkDerivation {
          name = "movement-repo";
          src = movement;
          dontBuild = true;
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
          '';
        };

        CONTAINER_REV = "c2372ff";

        # Helper function to pull Docker images with better error reporting
        pullDockerImages = ''
          # Exit on any error
          set -e

          # Main function that contains all the logic
          pull_docker_images() {
            echo "Pulling required Docker images..."
            
            # List of image names (without registry and tag)
            local image_names=(
              "movement-full-node"
              "movement-celestia-da-light-node"
              "movement-full-node-setup"
              "movement-faucet-service"
              "movement-celestia-bridge"
              "movement-celestia-appd"
              "wait-for-celestia-light-node"
            )

            # Function to construct full image name
            get_full_image_name() {
              local name=$1
              echo "ghcr.io/movementlabsxyz/$name:${CONTAINER_REV}"
            }
            
            # Function to pull a single image and track its status
            pull_image() {
              local image=$1
              echo -n "Pulling $image... "
              if docker pull "$image" > /dev/null 2>&1; then
                echo "✅"
                return 0
              else
                echo "❌"
                echo "  Error: Failed to pull $image"
                return 1
              fi
            }

            local failed=0
            local failed_images=()

            # Pull each image and track failures
            for name in "''${image_names[@]}"; do
              local full_image=$(get_full_image_name "$name")
              pull_image "$full_image" || { failed=1; failed_images+=("$name"); }
            done

            # Report summary
            if [ $failed -eq 1 ]; then
              echo -e "\nFailed to pull the following images:"
              for img in "''${failed_images[@]}"; do
                echo "  - $img"
              done
              echo -e "\nPlease check your Docker configuration and network connection."
              return 1
            else
              echo -e "\n✅ All images pulled successfully!"
            fi
          }

          # Execute the main function and exit if it fails
          pull_docker_images || exit 1
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

              set -e

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

              # Create symbolic link to movement repository
              mkdir -p .vendors
              ln -sfn ${movementRepo} .vendors/movement

              # Pull Docker images with better error reporting
              ${pullDockerImages}

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
