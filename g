#!/usr/bin/env bash

#set -eo pipefail

: "${GHC_DOWNLOAD_BASE_URL:="https://downloads.haskell.org/~ghc"}"
: "${G_PREFIX:="$HOME/haskell"}"
: "${OLD_DIR:=$(pwd)}"

usage() {
  USAGE=$(cat << END
g 0.2.0
The Haskell toolchain installer

USAGE:
    g [FLAGS] <SUBCOMMAND>

SUBCOMMANDS:
    install        Install a version of GHC
    switch         Switch to an installed version of GHC
    list           List all installed versions of GHC

DISCUSSION:
    g installs The Glorious Glasgow Haskell Compilation System,
    enabling you to easily switch between various versions of the
    compiler and keep them updated.
END
)
  echo "$USAGE"
}

os_to_target() {
  case "$1" in
    "darwin")
      echo "$(uname -m)-apple-darwin.+tar.xz$";
      ;;
    "linux")
      # N.B. This is tricky since most of the linux installs are non-standard.
      # FIXME this is currently passing a string as a regex but this assumption
      # shouldn't be made. Unless we decide tomake this a convention.
      echo "$(uname -m)(-deb[89]-linux|[^l]+linux-deb7)[^-]+tar.xz$"
      ;;
    *)
      exit 1
      ;;
  esac
}

cleanup() {
  TMP_DIR="$1"
  echo "Cleaning up ..."
  cd "$OLD_DIR" || exit;
  rm -rf "$TMP_DIR"
}

ghc_verify_checksums() {
  echo "Verifying checksums ... "
  local REMOTE_SHA256SUM="$2"
  local LOCAL_SHA256SUM
  LOCAL_SHA256SUM=$(shasum -a 256 "$1" | awk '{print $1}')

  if [ -z "$LOCAL_SHA256SUM" ] && \
     [ -z "$REMOTE_SHA256SUM" ] && \
     [ "$LOCAL_SHA256SUM" != "$REMOTE_SHA256SUM" ]; then
    echo "Checksums do not match"
    echo "   $REMOTE_SHA256SUM"
    echo "   $LOCAL_SHA256SUM"
    echo "Cleaning up and exiting"
    cleanup "$TMP_DIR"
    exit 1
  else
    echo "Checksums match"
    echo "   $REMOTE_SHA256SUM"
    echo "   $LOCAL_SHA256SUM"
  fi
}

ghc_install() {
  local FILE="$1"
  echo "Unpacking $(basename "$FILE") ... "
  tar xf "$FILE"

  rm "$FILE" # XXX Hack to let us grab the right directory in the next step.
  local DIR_NAME
  DIR_NAME=$(ls)
  cd "$DIR_NAME" || return 1
  local PREFIX="${G_PREFIX}/${DIR_NAME}"

  ./configure --prefix="$PREFIX"
  make install
}

ghc_download_and_install() {
  local GHC_VERSION="$1"
  local TMP_DIR
  TMP_DIR=$(mktemp -d)
  cd "$TMP_DIR" || exit;
  OS=$(uname | tr "[:upper:]" "[:lower:]")

  TARGET=$(os_to_target "$OS")

  BASE_URL="$GHC_DOWNLOAD_BASE_URL/${GHC_VERSION}"
  SHA256LINE=$(curl -s "$BASE_URL/SHA256SUMS" | grep -E "$TARGET" | head -1)

  if [ -z "$SHA256LINE" ]; then
    echo "FATAL: Could not find ghc $GHC_VERSION at remote."
    exit 1
  fi

  REMOTE_SHA256SUM=$( echo "$SHA256LINE" | awk '{print $1}')

  PACKAGE_NAME=$( echo "$SHA256LINE" | awk '{print $2}' | sed -e 's/^.\///')

  # FIXME This won't entirely work for rc and alpha's etc.
  # We could regex it out, instead.
  PREFIX=$(echo "$PACKAGE_NAME" | cut -d'-' -f1,2)
  echo "$PREFIX"
  # Check if we've installed this before.
  # TODO This check could probably be done way sooner.
  if [ -d "$PREFIX" ]; then
    echo "WARN: $DIR_NAME looks to already be present"
    echo "WARN: Aborting install"
    cleanup "$TMP_DIR"
    exit 1
  fi

  echo "Downloading ${PACKAGE_NAME} ..."
  TARGET_URL="${BASE_URL}/${PACKAGE_NAME}"
  if curl -O "$TARGET_URL"; then
    echo "Downloaded $PACKAGE_NAME successfully"
  else
    cleanup "$TMP_DIR"
    exit 1
  fi

  local DOWNLOAD="${TMP_DIR}/${PACKAGE_NAME}"

  ghc_verify_checksums "$DOWNLOAD" "$REMOTE_SHA256SUM"
  ghc_install "$DOWNLOAD"
  ghc_switch_version "$GHC_VERSION"

  cleanup "$TMP_DIR"
}

cabal_download_and_install() {
  if [ -z "$1" ]; then
    echo "FATAL: No version passed to \`cabal_download_and_install'"
    exit 1
  fi

  local TMP_DIR
  TMP_DIR=$(mktemp -d)
  cd "$TMP_DIR" || return 1

  VER="$1"
  curl -O "http://hackage.haskell.org/package/cabal-install-${VER}/cabal-install-${VER}.tar.gz"
  tar xf "cabal-install-$VER.tar.gz"
  cd "cabal-install-$VER" || return 1
  EXTRA_CONFIGURE_OPTS="" ./bootstrap.sh --sandbox --no-doc
  # $HOME/bin is assumed to exist and be on your $PATH
  # To install multiple cabal versions, we install the exact version
  # then symlink to it via the $HOME/bin/cabal ref.
  # TODO This will pave the way for switching between cabal versions easily.
  cp ".cabal-sandbox/bin/cabal" "$HOME/bin/cabal-$VER"
  ln -s -f "$HOME/bin/cabal-$VER" "$HOME/bin/cabal"

  # TODO Decide if we want to lock down the package index and unlock it on every cabal install?
  #$ chmod -R -w $HOME/.ghc/x86_64-darwin-<GHC_VERSION>/package.conf.d

  cleanup "$TMP_DIR"
}

ghc_list_available_versions() {
  echo "Available versions:"
  # TODO ignore the current dir.
  for ver in $G_PREFIX/ghc-*; do
    if [ ! "$(basename "$ver")" = "ghc-current" ]; then
      echo "  ${ver##$G_PREFIX/ghc-}"
    fi
  done
}

# TODO This should modify a symlink so the change is reflected outside of the script.
# We can have a general `$G_PREFIX/ghc-current/bin/*` that has symlinks to all appropriate binaries.
ghc_switch_version() {
  if [ -z "$1" ]; then
    echo "USAGE: g switch GHC_VERSION"
    ghc_list_available_versions
    return 1
  fi

  # TODO Need to check if the target versioln exists.

  VER_PATH="$G_PREFIX/ghc-$1"
  if [ -d "$VER_PATH" ]; then
    GHC_CURR_DIR="$G_PREFIX/ghc-current"
    if [ -d "$GHC_CURR_DIR" ]; then
      rm -rf "$GHC_CURR_DIR"
    fi
    mkdir -p "$GHC_CURR_DIR"
    for abs_d in $VER_PATH/*; do
      d=$(basename "$abs_d")
      ln -Fs "$VER_PATH/$d" "$GHC_CURR_DIR/$d"
    done
    ghc --version
  else
    echo "Cannot find installation for ghc version $1"
    exit 1
  fi
}

add_path_to_prefix() {
  local RC_CONF=""
  case $(basename "$SHELL") in
    "bash")
      RC_CONF=".bashrc"
      ;;
    "zsh")
      RC_CONF=".zshrc"
      ;;
    *)
      ;;
  esac

  if [ -z "$RC_CONF" ]; then
    echo "Please add \`export PATH=\$HOME/haskell/ghc-current/bin:\$PATH' to your shells configuration."
  else
    if ! grep -q "ghc-current" "$HOME/$RC_CONF"; then
      echo -e "export PATH=\"\$PATH:$G_PREFIX/ghc-current/bin\"\\n" >> "$HOME/$RC_CONF"
    fi
  fi
}

ghc_remove_version() {
  # TODO This doesn't do any validation on the input.
  VERSION="$1"
  TARGET="$G_PREFIX/ghc-$VERSION"
  if [ -d "$TARGET" ]; then
    echo "Removing $TARGET ... "
    rm -rf "$TARGET"
  else
    echo "GHC version $VERSION does not appear to be installed"
  fi
}

ghc_switch_to_next_version() {
  local CURR_GHC_VERSION
  CURR_GHC_VERSION=$(ghc --version | awk '{print $NF}')
  NEXT_GHC_VERSION=""

  local CHECK_NEXT=0
  for ver in $G_PREFIX/ghc-*; do
    # XXX Fake a do-while.
    if [ ! "$(basename "$ver")" = "ghc-current" ]; then
      if [ $CHECK_NEXT -eq 1 ]; then
        NEXT_GHC_VERSION="$(basename "$ver" | cut -d'-'  -f2)"
        CHECK_NEXT=0
        break;
      fi

      if [ "$(basename "$ver")" = "ghc-$CURR_GHC_VERSION" ]; then
        CHECK_NEXT=1
      fi
    fi
  done

  # Loop-around.
  if [ $CHECK_NEXT -eq 1 ]; then
    for ver in $G_PREFIX/ghc-*; do
      NEXT_GHC_VERSION="$(basename "$ver" | cut -d'-'  -f2)"
      break;
    done
  fi

  ghc_switch_version "$NEXT_GHC_VERSION"
}

main() {
  if [ $# -lt 1 ]; then
    ghc_switch_to_next_version
    exit 1
  else
    CMD="$1"
    case "$CMD" in
      "i" | "install")
        if [ $# -lt 2 ]; then
          echo "Please specify a version or \`latest' for installation"
          exit 1
        elif [ $# -eq 3 ] && [ "$2" = "--cabal" ]; then # XXX Super brittle.
          CABAL_VERSION="$3"
          echo "Checking if cabal version is already installed ... "
          # TODO This will need to change when we migrate over to doing a
          # `cabal-current` direct a la `ghc-current`.
          CURR_CABAL_VERSION=$(cabal --version | grep -Eo '([0-9]+\.){3}[0-9]+' | head -1)
          if which cabal &>/dev/null && [ "$CURR_CABAL_VERSION" = "$CABAL_VERSION" ]; then
            echo "Cabal version $CURR_CABAL_VERSION already installed"
            # TODO Per above, switch to version if cabal version is installed
            # but not presently the one running.
          else
            cabal_download_and_install "$CABAL_VERSION"
          fi
        else
          # GHC
          GHC_VERSION="$2"
          echo "Checking if ghc is present ..."
          found=0
          for abs_ver in $G_PREFIX/*; do
            ver=$(basename "$abs_ver")
            if [ "$ver" = "ghc-$GHC_VERSION" ]; then
              found=1
            fi
          done
          if [ $found -eq 0 ]; then
            ghc_download_and_install "$GHC_VERSION"
          else
            echo "ghc version $GHC_VERSION already installed"
          fi

          # CABAL
          echo "Checking if cabal is present ..."
          CURR_GHC_MAJ_VER=$(ghc --version | grep -Eo "([0-9]+\\.){2}[0-9]+$" | cut -d'.' -f1)
          CABAL_VERSION=$(if (( "$CURR_GHC_MAJ_VER" < 8 )); then echo "1.24.0.0"; else echo "2.0.0.1"; fi)
          if [ -z "$(which cabal)" ]; then
            echo "Can't find cabal; bootstrapping version $CABAL_VERSION"
            cabal_download_and_install "$CABAL_VERSION"
          else
            CURR_CABAL_VER=$(cabal --version | head -1 | grep -Eo "([0-9]+\\.){3}[0-9]+")
            echo "cabal version $CURR_CABAL_VER is already installed"
          fi

          add_path_to_prefix
        fi
        exit 0;
        ;;
      "l" | "list")
        ghc_list_available_versions
        exit 0
        ;;
      "r" | "remove")
        if [ $# -lt 2 ]; then
          echo 'Please specify a version for removal'
          exit 1
        else
          GHC_VERSION="$2"
          ghc_remove_version "$GHC_VERSION"
        fi
        exit 0;
        ;;
      "s" | "switch")
        if [ $# -lt 2 ]; then
          echo "Please specify version to switch to"
        else
          GHC_VERSION="$2"
          if [ -d "$G_PREFIX/${GHC_VERSION}" ]; then
            echo "Could not find ${GHC_VERSION} to switch to"
            exit 1
          else
            ghc_switch_version "$GHC_VERSION"
          fi
        fi
        exit 0
        ;;
      "-h" | "--help") # XXX Hack until we get proper argument parsing.
        usage
        exit 0
        ;;
      *)
        echo "Unrecognised command: ${CMD}"
        exit 1
        ;;
    esac
  fi

}

main "$@"
