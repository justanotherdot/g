#!/usr/bin/env bash

#set -eux

: "${GHC_DOWNLOAD_BASE_URL:="https://downloads.haskell.org/~ghc"}"
: "${G_PREFIX:="$HOME/haskell"}"
: "${OLD_DIR:=$(pwd)}"

usage() {
  USAGE=$(cat <<-END
g 0.1.0
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

    If you are new to Haskell consider ... to
    learn Haskell.
END
)
  echo "$USAGE"
}

os_to_target() {
  case "$OS" in
    "darwin")
      echo "x86_64-apple-darwin";
      ;;
    "linux")
      # N.B. This is tricky since most of the linux installs are non-standard.
      # FIXME this is currently passing a string as a regex but this assumption
      # shouldn't be made. Unless we decide tomake this a convention.
      echo "x86_64-deb[89]-linux[^-]"
      ;;
    *)
      echo "Platform unrecognised: ${OS}"
      exit 1
      ;;
  esac
}

ghc_verify_checksums() {
  echo "Verifying checksums ... "
  local REMOTE_SHA256SUM="$2"
  local LOCAL_SHA256SUM=$(shasum -a 256 "$1" | awk '{print $1}')

  if [ -z "$LOCAL_SHA256SUM" -a
       -z "$REMOTE_SHA256SUM" -a
       "$LOCAL_SHA256SUM" != "$REMOTE_SHA256SUM" ]; then
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

# FIXME all invocations of this should pass OLD_DIR explicitly
cleanup() {
  TMP_DIR="$1"
  OLD_DIR="${OLD_DIR:-$2}"
  echo "Cleaning up ..."
  cd "$OLD_DIR" || exit;
  rm -rvf "$TMP_DIR"
}

ghc_install() {
  local FILE="$1"
  echo "Unpacking $(basename "$FILE") ... "
  tar xf "$FILE"

  rm "$FILE" # XXX Hack to let us grab the right directory in the next step.
  local DIR_NAME=$(ls)
  cd "$DIR_NAME"
  local PREFIX="${G_PREFIX}/${DIR_NAME}"

  # Check if we've installed this before.
  # TODO This check could probably be done way sooner.
  if [ -d "$PREFIX" ]; then
    echo "$DIR_NAME looks to already be present"
    echo "Aborting install"
    cleanup "$TMP_DIR"
    exit 1
  fi

  ./configure --prefix="$PREFIX"
  make install
}

# Download and install a specific GHC version.
ghc_download_and_install() {
  local VERSION="$1"
  local TMP_DIR=$(mktemp -d)
  cd "$TMP_DIR" || exit;
  OS=$(uname | tr "[:upper:]" "[:lower:]")

  TARGET=$(os_to_target)

  BASE_URL="$GHC_DOWNLOAD_BASE_URL/${VERSION}"
  SHA256LINE=$(curl -s "$BASE_URL/SHA256SUMS" | egrep "$TARGET" | head -1)
  REMOTE_SHA256SUM=$( echo "$SHA256LINE" | awk '{print $1}')

  PACKAGE_NAME=$( echo "$SHA256LINE" | awk '{print $2}' | sed -e 's/^.\///')

  echo "Downloading ${PACKAGE_NAME} ..."
  TARGET_URL="${BASE_URL}/${PACKAGE_NAME}"
  curl -O "$TARGET_URL"
  if [ $? -eq 0 ]; then
    echo "Downloaded $PACKAGE_NAME successfully"
  else
    cleanup "$TMP_DIR"
    exit 1
  fi

  local DOWNLOAD="${TMP_DIR}/${PACKAGE_NAME}"

  ghc_verify_checksums "$DOWNLOAD" "$REMOTE_SHA256SUM"
  ghc_install "$DOWNLOAD"

  cleanup "$TMP_DIR"
}

ghc_list_available_versions() {
  echo "Available versions:"
  for ver in $G_PREFIX/ghc-*; do
    echo "  ${ver##$G_PREFIX/ghc-}"
  done
}

remove_ghc_from_path() {
    # set the Internal Field Separator to be ':'
    # see http://www.tldp.org/LDP/abs/html/internalvariables.html
    IFS=:
    # convert it to an array
    t=($PATH)
    unset IFS
    # remove elements with ghc from the array
    t=(${t[@]%%*ghc*})
    IFS=:
    # set the path to the new array
    PATH="${t[*]}"
    unset IFS
}

ghc_switch_version() {
  if [ -z "$1" ]; then
    echo "USAGE: g switch VERSION"
    ghc_list_available_versions
    return 1
  fi

  VER_PATH="$G_PREFIX/ghc-$1"
  if [ -d "$VER_PATH" ]; then
    remove_ghc_from_path
    PATH="$VER_PATH/bin:$PATH"
    export GHC_VERSION="$1"
    ghc --version
  else
    echo "GHC $1 isn't available"
    ghc_list_available_versions
    return 1
  fi
}

main() {
  if [ $# -lt 1 ]; then
    usage
    exit 1
  else
    CMD="$1"
    case "$CMD" in
      "i" | "install")
        if [ $# -lt 2 ]; then
          echo 'Please specify a specific version or `latest` for installation'
          exit 1
        else
          VERSION="$2"
          ghc_download_and_install "$VERSION"
        fi
        ;;
      "l" | "list")
        ghc_list_available_versions
        exit 1
        ;;
      "s" | "switch")
        if [ $# -lt 2 ]; then
          echo "Please specify version to switch to"
        else
          VERSION="$2"
          if [ -d "$G_PREFIX/${VERSION}" ]; then
            echo "Could not find ${VERSION} to switch to"
            exit 1
          else
            ghc_switch_version "$VERSION"
          fi
        fi
        exit 1
        ;;
      *)
        echo "Unrecognised command: ${CMD}"
        ;;
    esac
  fi


  # TODO Do we care about cycling like this if we're going to do some sort of menu or similar?
  ## Switch to a specific GHC version
  #ghc-switch() {
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


  # CABAL INSTALL
  ## Change this value to the appropriate one
  #VER=1.24.0.0
  #wget "http://hackage.haskell.org/package/cabal-install-${VER}/cabal-install-${VER}.tar.gz"
  #tar xf cabal-install-$VER.tar.gz
  #cd cabal-install-$VER
  #EXTRA_CONFIGURE_OPTS="" ./bootstrap.sh --sandbox --no-doc
  ## $HOME/bin is assumed to exist and be on your $PATH
  #cp .cabal-sandbox/bin/cabal $HOME/bin/cabal

  # TODO Decide if we want to lock down the package index and unlock it on every cabal install?
  #$ chmod -R -w $HOME/.ghc/x86_64-darwin-<GHC_VERSION>/package.conf.d

  # SETS PATH
  # $ export PATH=$HOME/haskell/ghc-7.10.2/bin:$PATH
  # TODO Best if we actually appended this to the current shell's rc.

  # TODO usage function
  # TODO usage specifies you can download by version
  # TODO take a prefix for install location, defaults to under home with ~/.ghc and ~/.cabal
}

main "$@"
