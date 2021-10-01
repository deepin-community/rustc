#!/bin/bash
set -e

ver="$1"
dfsg="${2:-+dfsg1}"
upstream_tag="upstream/${ver/\~/_}${dfsg/\~/_}"

git show -s upstream/experimental
git show -s debian/experimental
printf "\ngit top-level dir: %s\n" "$(git rev-parse --show-toplevel)"
printf "version: $ver\n"

if ! git merge-base --is-ancestor  upstream/experimental debian/experimental; then
    echo >&2 "upstream/experimental is not an ancestor of debian/experimental"
fi
if git rev-parse "${upstream_tag}" 2>/dev/null >/dev/null; then
    echo >&2 "tag already exists: ${upstream_tag}"
fi

read -p "continue? [y/N] " x
if [ "$x" != "y" ]; then exit 1; fi

cd "$(git rev-parse --show-toplevel)"
git branch -f upstream/rebase-patches upstream/experimental
git branch -f debian/rebase-patches debian/experimental
git checkout debian/rebase-patches

git branch -f patch-queue/debian/rebase-patches
for i in debian/patches/d-00*.patch; do gbp pq apply "$i"; done

gbp import-orig "../rustc_${ver}${dfsg}.orig.tar.xz" \
  --upstream-branch=upstream/rebase-patches \
  --debian-branch=debian/rebase-patches \
  --no-sign-tags --no-pristine-tar --no-symlink-orig

# rebase here
echo "$0: Now manually rebase - run 'git rebase debian/rebase-patches'"
echo "$0: There may be conflicts; follow the instructions that git tells you."
echo "$0: When done, exit the child shell with ctrl-D"
$SHELL

gbp pq export --no-patch-numbers
for i in debian/patches/d-00*.patch; do git add "$i"; done
git commit -m "Update early-stage patches for ${ver}${dfsg}"
git checkout .
git rebase @~ --onto=debian/experimental
git branch -f debian/experimental
git checkout debian/experimental

# cleanup
git tag -d "${upstream_tag}" || true
git branch -D upstream/rebase-patches || true
git branch -D debian/rebase-patches || true
git branch -D patch-queue/debian/rebase-patches || true
