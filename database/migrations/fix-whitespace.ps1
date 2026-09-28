Get-ChildItem -Filter *.sql | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $original = $content
    
    # Remove trailing commas before closing parenthesis
    $content = $content -replace ',(\s*)\)', '$1)'
    
    if ($content -ne $original) {
        Set-Content -Path $_.FullName -Value $content -NoNewline
        Write-Host "Fixed: $($_.Name)" -ForegroundColor Green
    }
}

Write-Host "`nDone! All trailing commas removed." -ForegroundColor Cyan