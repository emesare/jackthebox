# Create bin dir if not exist
if (!(Test-Path "bin")) {
    New-Item "bin" -Type Directory
}

# Copy all referenced assemblies from installed s&box.
if (Test-Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Steam App 590830\') {
    $path = Get-ItemPropertyValue -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Steam App 590830\' -Name InstallLocation
    Get-ChildItem -Path $path -Recurse |
    Where-Object Name -like "Sandbox*.dll" | 
    Copy-Item -Destination "bin\" -Force
}
else {
    # S&box not installed, running script in CI, Display available assemblies.
    Get-ChildItem -Path "bin\" -Recurse -Include "Sandbox*.dll" | Format-Table -Property Name, Length, LastWriteTime
}