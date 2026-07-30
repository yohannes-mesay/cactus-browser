#!/usr/bin/env bash
# Bake resources/landing.html into a static site/ folder for GitHub Pages.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${ROOT}/site"
SRC="${ROOT}/resources/landing.html"

SITE_URL="${SITE_URL:-https://cactus-browser.tech}"
GITHUB_REPO="${GITHUB_REPO:-https://github.com/yohannes-mesay/cactus-browser}"
APP_VERSION="${APP_VERSION:-1.2.0}"
WEBKIT_VERSION="${WEBKIT_VERSION:-6}"

mkdir -p "$OUT"
cp "$SRC" "$OUT/index.html"

replace() {
  local from="$1"
  local to="$2"
  sed -i "s|${from}|${to}|g" "$OUT/index.html"
}

replace '{{SITE_URL}}' "$SITE_URL"
replace '{{GITHUB_REPO}}' "$GITHUB_REPO"
replace '{{GITHUB_ISSUES}}' "${GITHUB_REPO}/issues"
replace '{{GITHUB_RELEASES}}' "${GITHUB_REPO}/releases"
replace '{{APP_VERSION}}' "$APP_VERSION"
replace '{{WEBKIT_VERSION}}' "$WEBKIT_VERSION"

printf '%s\n' 'cactus-browser.tech' > "$OUT/CNAME"

echo "Wrote ${OUT}/index.html and ${OUT}/CNAME"
