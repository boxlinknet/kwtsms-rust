#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>  (e.g. $0 0.2.0)"
  exit 1
fi

VERSION="$1"
TAG="v${VERSION}"

if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Error: tag $TAG already exists"
  exit 1
fi

if [ -n "$(git status --porcelain)" ]; then
  echo "Error: working tree is dirty. Commit or stash changes first."
  exit 1
fi

sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

cargo check --quiet

git add Cargo.toml
git commit -m "chore: bump v${VERSION}"
git tag "$TAG"
git push
git push origin "$TAG"

echo ""
echo "Released ${TAG}. CI will publish to crates.io and create a GitHub Release."
