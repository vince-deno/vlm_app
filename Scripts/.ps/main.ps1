param (
    [string]$scriptPath
)

# Validate the `.sh` file exists before running
if (!(Test-Path $scriptPath)) {
    Write-Host "Error: The specified .sh file does not exist: $scriptPath"
    exit 1
}

# Internal Test: Check if file is a valid shell script
$firstLine = Get-Content -Path $scriptPath -TotalCount 1
if ($firstLine -notmatch "^#!.*sh") {
    Write-Host "Warning: The script does not start with a shebang (#!). It may not be a valid shell script."
}

# Detect shell environment
if (Get-Command wsl -ErrorAction SilentlyContinue) {
    Write-Host "Running script using WSL..."
    wsl bash -c "`"$scriptPath`""
} 
elseif (Test-Path "C:\Program Files\Git\bin\bash.exe") {
    Write-Host "Running script using Git Bash..."
    & "C:\Program Files\Git\bin\bash.exe" -c "`"$scriptPath`""
} 
elseif (Test-Path "C:\cygwin64\bin\bash.exe") {
    Write-Host "Running script using Cygwin..."
    & "C:\cygwin64\bin\bash.exe" -c "`"$scriptPath`""
} 
else {
    Write-Host "Error: No compatible shell found! Install WSL, Git Bash, or Cygwin."
    exit 1
}
