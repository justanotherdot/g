#!/usr/bin/env bash

#set -eux

: "${GHC_DOWNLOAD_BASE_URL:="https://downloads.haskell.org/~ghc"}"
: "${G_PREFIX:="$HOME/haskell"}"
: "${VERSION:="latest"}"
: "${OLD_DIR:=$(pwd)}"

os_to_target() {
  case "$OS" in
    "darwin")
      echo "x86_64-apple-darwin";
      ;;
    "linux")
      # N.B.
      # This is tricky since most of the linux installs are non-standard.
      # It looks
      echo "deb9-linux"
      ;;
    *)
      echo "Platform unrecognised: ${OS}"
      exit 1
      ;;
  esac
}

cleanup() {
  local TMP_DIR="$1"
  local OLD_DIR="${OLD_DIR:-$2}"
  echo "Cleaning up ..."
  cd "$OLD_DIR" || exit;
  rm -rvf "$TMP_DIR"
}

main() {
  TMP_DIR=$(mktemp -d)
  cd "$TMP_DIR" || exit;
  OS=$(uname | tr "[:upper:]" "[:lower:]")

  TARGET=$(os_to_target)

  if [[ $# -lt 1 ]]; then
    # FIXME this currently errors out
    # but normally would just show all versions.
    echo "No command given, quitting"
    exit 1
  else
    CMD="$1"
    case "$CMD" in
      "i" | "install")
        if [[ $# -lt 2 ]]; then
          echo "Version not passed, defaulting to ${VERSION}"
        else
          VERSION="$2"
          echo "$VERSION"
        fi
        ;;
      *)
        echo "Unrecognised command: ${CMD}"
        ;;
    esac
  fi

  # SUMS
  BASE_URL="$GHC_DOWNLOAD_BASE_URL/${VERSION}"
  SHA256LINE=$(curl -s "$BASE_URL/SHA256SUMS" | grep "$TARGET")
  REMOTE_SHA256SUM=$( echo "$SHA256LINE" | awk '{print $1}')

  PACKAGE_NAME=$( echo "$SHA256LINE" | awk '{print $2}' | sed -e 's/^.\///')

  echo "Downloading ${PACKAGE_NAME} ..."
  TARGET_URL="${BASE_URL}/${PACKAGE_NAME}"
  curl -O "${TARGET_URL}"

  LOCAL_SHA256SUM=$(shasum -a 256 "${TMP_DIR}/${PACKAGE_NAME}" | awk '{print $1}')

  if [[ "$LOCAL_SHA256SUM" != "$REMOTE_SHA256SUM" ]]; then
    echo "Checksums do not match"
    echo "Cleaning up and exiting"
    cleanup "$TMP_DIR"
  else
    echo "Checksums match"
  fi


  # TODO SOME SHORTCUTS `G` SHOULD BE ABLE TO DO.
  ## List available GHC versions
  #ghc-list-available() {
  #  echo "Available versions:"
  #  for ver in $HOME/haskell/ghc-*; do
  #    echo "  ${ver##$HOME/haskell/ghc-}"
  #  done
  #}
  #
  ## Switch to a specific GHC version
  #ghc-switch() {
  #  if [ -z "$1" ]; then
  #    echo "USAGE: ghc-switch VERSION"
  #    ghc-list-available
  #    return 1
  #  fi
  #
  #  VER_PATH="$HOME/haskell/ghc-$1"
  #  if [ -d "$VER_PATH" ]; then
  #    export path=($VER_PATH/bin ${(@)path:#*ghc*})
  #    export GHC_VERSION=$1
  #    ghc --version
  #  else
  #    echo "GHC $1 isn't available"
  #    ghc-list-available
  #    return 1
  #  fi
  #}
  #
  ## Cycle GHC versions
  #g() {
  #  case $GHC_VERSION in
  #    7.8.4)
  #      ghc-switch 7.10.2
  #      ;;
  #    *)
  #      ghc-switch 7.8.4
  #      ;;
  #  esac
  #}

  # TODO Will need to consider OS
  # GHC INSTALL
  #$ wget https://downloads.haskell.org/~ghc/7.10.2/ghc-7.10.2-x86_64-apple-darwin.tar.xz
  #$ tar xf ghc-7.10.2-x86_64-apple-darwin.tar.xz
  #$ cd ghc-7.10.2
  #$ ./configure --prefix=$HOME/haskell/ghc-7.10.2
  #$ make install

  # CABAL INSTALL
  ## double-check that we're installing the right version of the Cabal library
  #cabal --version
  #> cabal-install version 1.24.0.0
  #> using version 1.24.0.0 of the Cabal library
  ## make sure we're not in a sandbox, jumping to $HOME is a safe bet
  #cd
  #cabal install Cabal-1.24.0.0

  # TODO Decide if we want to lock down the package index and unlock it on every cabal install?
  #$ chmod -R -w $HOME/.ghc/x86_64-darwin-<GHC_VERSION>/package.conf.d

  # SETS PATH
  # $ export PATH=$HOME/haskell/ghc-7.10.2/bin:$PATH
  # TODO Best if we actually appended this to the current shell's rc.

  # TODO usage function
  # TODO usage provides a `latest` option
  # TODO usage specifies you can download by version
  # TODO take a prefix for install location, defaults to under home with ~/.ghc and ~/.cabal
  cleanup "$TMP_DIR"
}

main "$@"
