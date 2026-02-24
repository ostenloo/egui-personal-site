#!/usr/bin/env bash
set -euo pipefail

DIR="blog_posts"
mkdir -p "$DIR"

# 1) Ask for title
read -rp "Post title: " TITLE

# 2) Generate ISO-8601 timestamp with colon in timezone offset
#    GNU date supports %:z, BSD does not.
if TS=$(date +"%Y-%m-%dT%H:%M:%S%:z" 2>/dev/null); then
  : # Successful GNU format
else
  BASE=$(date +"%Y-%m-%dT%H:%M:%S")
  OFF=$(date +%z)             # e.g. "-0600"
  OFF="${OFF:0:3}:${OFF:3:2}" # '-06:00'
  TS="$BASE$OFF"
fi

# 3) Create slug
SLUG=$(printf '%s' "$TITLE" \
  | tr '[:upper:]' '[:lower:]' \
  | sed -E 's/[^a-z0-9]+/-/g; s/^-+|-+$//g')

# 4) Timestamp-based unique filename
TIMESTAMP=$(date +"%Y-%m-%d-%H%M%S")
FNAME="$DIR/${TIMESTAMP}-${SLUG:-post}.md"

i=0
while [ -e "$FNAME" ]; do
  i=$((i+1))
  FNAME="$DIR/${TIMESTAMP}-${SLUG:-post}-$i.md"
done

# Escape quotes for YAML
ESC_TITLE=$(printf '%s' "$TITLE" | sed 's/"/\\"/g')

# 5) Write frontmatter
cat > "$FNAME" <<EOF
---
title: "$ESC_TITLE"
date: "$TS"
---
EOF

echo "Created: $FNAME"
