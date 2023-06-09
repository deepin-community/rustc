#!/bin/bash
set -e

ver="$1"
test -n "$ver" || exit 2

SUS_WHITELIST=$(find "${PWD}/debian" -name upstream-tarball-unsuspicious.txt -type f)

rm -rf rustc-${ver/*~*/beta}-src/
tar xf ../rustc_$ver+dfsg1.orig.tar.xz && cd rustc-${ver/*~*/beta}-src/

# TODO: remove this code snippet after it gets into our cargo
# Strip comments & blank lines before testing rust source code -
# some authors like to write really long comments
find . -name '*.rs' -execdir sed -i -e '\,^\s*//,d' -e '/^\s*$/d' '{}' \;

/usr/share/cargo/scripts/audit-vendor-source \
  "$SUS_WHITELIST" \
  "Files-Excluded: in debian/copyright and run a repack." \
  -m text/x-script.python \
  -m application/csv

echo "Artifacts left in rustc-$ver-src, please remove them yourself."
