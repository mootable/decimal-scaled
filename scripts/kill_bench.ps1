#requires -Version 7
Get-Process |
    Where-Object { $_.ProcessName -match 'cargo|full_matrix|lib_cmp|m2_' } |
    ForEach-Object {
        Write-Host "killing $($_.ProcessName) PID=$($_.Id)"
        Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
    }
