# Starts an installed TeachPad on Windows and checks that it works.
#
# 1. The app prints its address.
# 2. It serves its page and its script on that address.
# 3. The page inside its window connects back to the app.
#
# The third step is the one that shows the window works: the app prints its
# address and serves its page before a window exists. The server logs
# "socket opened" for each socket, and the page in the window opens the
# first one. launch.sh is the same check for macOS and Linux.
#
# Usage: launch.ps1 -App <path to teachpad.exe>
param([Parameter(Mandatory = $true)][string]$App)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

# How long each wait may take, in seconds. On a fresh machine the first
# start sets up the web view's profile, which takes most of that time.
$limit = if ($env:LAUNCH_LIMIT) { [int]$env:LAUNCH_LIMIT } else { 120 }
$logs = Join-Path ([IO.Path]::GetTempPath()) ('teachpad-launch-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force -Path $logs | Out-Null
$stdout = Join-Path $logs 'stdout.log'
$stderr = Join-Path $logs 'stderr.log'

function Fail([string]$Message) {
  foreach ($file in $stdout, $stderr) {
    Write-Host "--- the last lines of the app's $(Split-Path $file -Leaf)"
    if (Test-Path -LiteralPath $file) { Get-Content -LiteralPath $file -Tail 40 | Write-Host }
  }
  throw "launch: $Message"
}

if (-not (Test-Path -LiteralPath $App)) { throw "launch: there is no app at $App" }
$process = Start-Process -FilePath $App -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
# With redirected streams, ExitCode stays empty unless the handle is read
# while the process runs.
$null = $process.Handle
try {
  # 1. The app prints its address.
  $url = $null
  foreach ($attempt in 1..$limit) {
    $line = Get-Content -LiteralPath $stdout -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($line -match '^http://127\.0\.0\.1:\d+/$') { $url = $line; break }
    if ($process.HasExited) { Fail "the app exited with code $($process.ExitCode) before it printed its address" }
    Start-Sleep -Seconds 1
  }
  if (-not $url) { Fail "the app printed no address within $limit seconds" }

  # 2. It serves its page and its script.
  try {
    $page = Invoke-WebRequest -UseBasicParsing -Uri $url -TimeoutSec 30
  } catch {
    Fail "$url did not answer: $($_.Exception.Message)"
  }
  if (-not "$($page.Content)".Contains('src="/app.js"')) { Fail "the page at $url does not load /app.js" }
  try {
    $script = Invoke-WebRequest -UseBasicParsing -Uri ($url + 'app.js') -TimeoutSec 30
  } catch {
    Fail "$($url)app.js did not answer: $($_.Exception.Message)"
  }
  if ($script.RawContentLength -le 0) { Fail "$($url)app.js is empty" }

  # 3. The page in the window connects back. The two requests above open
  # no socket, so the first "socket opened" comes from the window.
  $connected = $false
  foreach ($attempt in 1..$limit) {
    if (Select-String -LiteralPath $stderr -SimpleMatch 'socket opened' -Quiet) { $connected = $true; break }
    if ($process.HasExited) { Fail "the app exited with code $($process.ExitCode) before the page in its window connected" }
    Start-Sleep -Seconds 1
  }
  if (-not $connected) { Fail "the page in the window did not connect within $limit seconds" }
  if ($process.HasExited) { Fail "the app exited with code $($process.ExitCode) right after the page in its window connected" }

  Write-Host "launch: the app serves $url, and the page in its window is connected"
} finally {
  if (-not $process.HasExited) {
    # /T stops the helper processes of the web view with the app.
    & taskkill /PID $process.Id /T /F 2>&1 | Out-Null
  }
}
