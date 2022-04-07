#!/usr/bin/env bash
set -evuxo pipefail

red=`tput setaf 1`
green=`tput setaf 2`
reset=`tput sgr0`

cd "$(dirname $0)"

status() {
    echo "$@${reset}" 1>&2
}

# armeabi-v7a arm64-v8a x86 x86_64
ARCH="${1:-armeabi-v7a}"
NDK_API_LEVEL=21
FILE="./out/${ARCH}/libarmv7_problems.so"

cargo ndk \
    --bindgen -t "${ARCH}" -p "${NDK_API_LEVEL}" \
    -o "./out" \
    build \
    --locked

SCAN_RESULT="$(scanelf -qT "$FILE")"

if [[ ! -z "$SCAN_RESULT" ]]; then
    status "${red}SO with relocations"
    status "$SCAN_RESULT"
    exit 1
else
    status "${green}no relocations found"
fi

# patchelf --set-soname libarmv7_problems.so out/armeabi-v7a/libarmv7_problems.so

if readelf --dynamic "$FILE" | grep SONAME; then
    status "${green}SO has SONAME"
else
    status "${red}SO misses SONAME"
    exit 1
fi
