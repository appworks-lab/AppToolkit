#!/bin/bash

# This file is based on: https://github.com/Schniz/fnm/blob/master/.ci/install.sh

set -e

VERSION="cli-v0.0.0-beta.5"
OS="$(uname -s)"

case "${OS}" in
   MINGW* | Win*) OS="Windows" ;;
esac

if [ -d "$HOME/.toolkit" ]; then
  INSTALL_DIR="$HOME/.toolkit"
elif [ -n "$XDG_DATA_HOME" ]; then
  INSTALL_DIR="$XDG_DATA_HOME/toolkit"
elif [ "$OS" = "Darwin" ]; then
  INSTALL_DIR="$HOME/Library/Application Support/toolkit"
else
  INSTALL_DIR="$HOME/.local/share/toolkit"
fi

# Parse Flags
parse_args() {
  while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
    --install)
      COMMAND="install"
      shift # past argument
      ;;
    --list)
      COMMAND="list"
      shift # past argument
      ;;
    --manifest)
      MANIFEST_PATH="$2"
      shift # past argument
      shift # past value
      ;;
    *)
      echo "Unrecognized argument $key"
      exit 1
      ;;
    esac
  done
}

set_filename() {
  if [ "$OS" = "Linux" ]; then
    # TODO: Support Linux
    # Based on https://stackoverflow.com/a/45125525
    case "$(uname -m)" in
      arm | armv7*)
        FILENAME="toolkit-linux-arm32"
        ;;
      aarch* | armv8*)
        FILENAME="toolkit-linux-arm64"
        ;;
      *)
        FILENAME="toolkit-linux"
    esac
  elif [ "$OS" = "Darwin" ]; then
    case "$(uname -m)" in
      arm64)
        FILENAME="toolkit-macOS-aarch64"
        ;;
      *)
        FILENAME="toolkit-macOS-x86_64"
    esac
  elif [ "$OS" = "Windows" ]; then
    FILENAME="toolkit-Windows-x86_64"
  else
    echo "OS $OS is not supported."
    echo "If you think that's a bug - please file an issue"
    exit 1
  fi
}

download_toolkit() {
    URL="https://github.com/apptools-lab/AppToolkit/releases/download/$VERSION/$FILENAME.zip"

    DOWNLOAD_DIR=$(mktemp -d)

    echo "Downloading $URL..."

    mkdir -p "$INSTALL_DIR" &>/dev/null

    if ! curl --progress-bar --fail -L "$URL" -o "$DOWNLOAD_DIR/$FILENAME.zip"; then
        echo "Download failed.  Check that the release/filename are correct."
        exit 1
    fi

    unzip -q "$DOWNLOAD_DIR/$FILENAME.zip" -d "$DOWNLOAD_DIR"

    if [ -f "$DOWNLOAD_DIR/toolkit" ]; then
        mv "$DOWNLOAD_DIR/toolkit" "$INSTALL_DIR/toolkit"
    else
        mv "$DOWNLOAD_DIR/$FILENAME/toolkit" "$INSTALL_DIR/toolkit"
    fi

    chmod u+x "$INSTALL_DIR/toolkit"
}

setup_shell() {
    if [ -z "$MANIFEST_PATH" ]; then
        "$INSTALL_DIR/toolkit" $COMMAND
    else
        "$INSTALL_DIR/toolkit" $COMMAND --manifest "$MANIFEST_PATH"
    fi
}

parse_args "$@"
set_filename
download_toolkit
setup_shell