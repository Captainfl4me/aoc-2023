$day = $args[0]
$cmd = "cargo generate .\template-day\ --name day-$day -d day=$day"
Invoke-Expression $cmd
Write-Host "New day: $day"
