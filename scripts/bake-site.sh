#!/usr/bin/env bash
# Bake resources/landing.html into a static site/ folder for GitHub Pages.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${ROOT}/site"
SRC="${ROOT}/resources/landing.html"
WEB="${ROOT}/resources/web"

SITE_URL="${SITE_URL:-https://cactus-browser.tech}"
GITHUB_REPO="${GITHUB_REPO:-https://github.com/yohannes-mesay/cactus-browser}"
APP_VERSION="${APP_VERSION:-1.2.0}"
WEBKIT_VERSION="${WEBKIT_VERSION:-6}"

mkdir -p "$OUT"
cp "$SRC" "$OUT/index.html"
cp "$WEB/favicon.svg" "$OUT/favicon.svg"
cp "$WEB/site.webmanifest" "$OUT/site.webmanifest"
cp "$WEB/robots.txt" "$OUT/robots.txt"
cp "$WEB/sitemap.xml" "$OUT/sitemap.xml"

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

generate_pngs() {
  local svg="$OUT/favicon.svg"
  if command -v rsvg-convert >/dev/null 2>&1; then
    rsvg-convert -w 16 -h 16 "$svg" -o "$OUT/favicon-16.png"
    rsvg-convert -w 32 -h 32 "$svg" -o "$OUT/favicon-32.png"
    rsvg-convert -w 180 -h 180 "$svg" -o "$OUT/apple-touch-icon.png"
    rsvg-convert -w 512 -h 512 "$svg" -o "$OUT/icon-512.png"
    # Open Graph / social preview
    rsvg-convert -w 400 -h 400 "$svg" -o "$OUT/_og-icon.png"
    if command -v convert >/dev/null 2>&1; then
      convert -size 1200x630 xc:'#f2ebe1' \
        "$OUT/_og-icon.png" -gravity center -composite \
        "$OUT/og-image.png"
      rm -f "$OUT/_og-icon.png"
    else
      cp "$OUT/icon-512.png" "$OUT/og-image.png"
    fi
    return 0
  fi
  if command -v convert >/dev/null 2>&1; then
    convert -background none -density 256 "$svg" -resize 16x16 "$OUT/favicon-16.png"
    convert -background none -density 256 "$svg" -resize 32x32 "$OUT/favicon-32.png"
    convert -background none -density 256 "$svg" -resize 180x180 "$OUT/apple-touch-icon.png"
    convert -background none -density 256 "$svg" -resize 512x512 "$OUT/icon-512.png"
    convert -size 1200x630 xc:'#f2ebe1' \
      \( "$OUT/icon-512.png" -resize 420x420 \) -gravity center -composite \
      "$OUT/og-image.png"
    return 0
  fi
  return 1
}

if ! generate_pngs; then
  echo "warning: rsvg-convert/convert not found; using committed PNG fallbacks if present" >&2
  for f in favicon-16.png favicon-32.png apple-touch-icon.png icon-512.png og-image.png; do
    if [[ -f "$WEB/$f" ]]; then
      cp "$WEB/$f" "$OUT/$f"
    fi
  done
fi

echo "Wrote ${OUT}/ (index.html, CNAME, icons, manifest)"
