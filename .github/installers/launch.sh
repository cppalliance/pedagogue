#!/usr/bin/env bash
# Starts an installed Pedagogue on macOS or Linux and checks that it works.
#
# 1. The app prints its address.
# 2. It serves its page and its script on that address.
# 3. The page inside its window connects back to the app.
#
# The third step is the one that shows the window works: the app prints its
# address and serves its page before a window exists. The server logs
# "socket opened" for each socket, and the page in the window opens the
# first one. launch.ps1 is the same check for Windows.
#
# Usage: launch.sh <command> [<argument> ...]
#   launch.sh /Applications/Pedagogue.app/Contents/MacOS/pedagogue
#   launch.sh xvfb-run -a pedagogue
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "usage: launch.sh <command> [<argument> ...]" >&2
  exit 2
fi

# How long each wait may take, in seconds. On a fresh machine the first
# start sets up the web view's profile, which takes most of that time.
limit=${LAUNCH_LIMIT:-120}
logs=$(mktemp -d)

fail() {
  echo "launch: $*" >&2
  for stream in stdout stderr; do
    echo "--- the last lines of the app's $stream" >&2
    tail -n 40 "$logs/$stream.log" >&2 || true
  done
  exit 1
}

# Job control gives the app a process group of its own, so the helper
# processes of the web view, and xvfb-run with its X server, stop with it.
set -m
"$@" >"$logs/stdout.log" 2>"$logs/stderr.log" &
app=$!
set +m

stop() {
  kill -- "-$app" 2>/dev/null || kill "$app" 2>/dev/null || true
}
trap stop EXIT

running() {
  kill -0 "$app" 2>/dev/null
}

# 1. The app prints its address.
url=""
for _ in $(seq 1 "$limit"); do
  url=$(grep -m 1 -E '^http://127\.0\.0\.1:[0-9]+/$' "$logs/stdout.log" || true)
  [ -z "$url" ] || break
  running || fail "the app exited before it printed its address"
  sleep 1
done
[ -n "$url" ] || fail "the app printed no address within $limit seconds"

# 2. It serves its page and its script.
curl -sf --max-time 30 "$url" -o "$logs/index.html" || fail "$url did not answer"
grep -Fq 'src="/app.js"' "$logs/index.html" || fail "the page at $url does not load /app.js"
curl -sf --max-time 30 "${url}app.js" -o "$logs/app.js" || fail "${url}app.js did not answer"
[ -s "$logs/app.js" ] || fail "${url}app.js is empty"

# 3. The page in the window connects back. The two requests above open no
# socket, so the first "socket opened" comes from the window.
connected=""
for _ in $(seq 1 "$limit"); do
  if grep -Fq 'socket opened' "$logs/stderr.log"; then
    connected=yes
    break
  fi
  running || fail "the app exited before the page in its window connected"
  sleep 1
done
[ -n "$connected" ] || fail "the page in the window did not connect within $limit seconds"
running || fail "the app exited right after the page in its window connected"

echo "launch: the app serves $url, and the page in its window is connected"
