#!/usr/bin/env bash
set -euo pipefail

DIR="blog_posts"
mkdir -p "$DIR"

# 1) Ask for title
read -rp "Post title: " TITLE

# 2) Make an ISO-8601 local timestamp with colon in the timezone offset
#    GNU date supports %:z; BSD/macOS doesn't, so we fall back and insert the colon.
if TS=$(date +"%Y-%m-%dT%H:%M:%S%:z" 2>/dev/null); then
  : # GNU date path
else
  BASE=$(date +"%Y-%m-%dT%H:%M:%S")
  OFF=$(date +%z)                   # e.g., -0400
  OFF="${OFF:0:3}:${OFF:3:2}"       # -> -04:00
  TS="$BASE$OFF"
fi

# 3) Slugify filename from title (lowercase, hyphens, strip non-alnum)
SLUG=$(printf '%s' "$TITLE" \
  | tr '[:upper:]' '[:lower:]' \
  | sed -E 's/[^a-z0-9]+/-/g; s/^-+|-+$//g')

TIMESTAMP=$(date +"%Y-%m-%d-%H%M%S")
FNAME="$DIR/${TIMESTAMP}-${SLUG:-post}.md"

# Ensure uniqueness (append -1, -2, ... if needed)
i=0
while [ -e "$FNAME" ]; do
  i=$((i+1))
  FNAME="$DIR/${TIMESTAMP}-${SLUG:-post}-$i.md"
done

# Escape double quotes in title for YAML
ESC_TITLE=$(printf '%s' "$TITLE" | sed 's/"/\\"/g')

# 4) Write frontmatter
cat > "$FNAME" <<EOF
---
title: "$ESC_TITLE"
date: "$TS"
---
EOF

echo "Created: $FNAME"
