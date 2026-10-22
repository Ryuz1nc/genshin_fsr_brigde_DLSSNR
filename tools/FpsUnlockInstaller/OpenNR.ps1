# OpenNR ReShade add-on v0.7.14 and AMD runtime v0.6.0.
function Install-OpenNrPayload {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$ReShadeDirectory,
        [Parameter(Mandatory)][string]$GameDirectory
    )

    $assets = @(
        @{ RelativePath = 'reshade-shaders\Addons\amd-nr.addon64'; Size = 732672; Hash = '860302A29E370194C57118E11A51B0B8402302702BE7A1FC70F6CDC72283E8B5'; Install = $false },
        @{ RelativePath = 'reshade-shaders\Shaders\AMD_Neural_Feed.fx'; Size = 18395; Hash = 'A11BB2FB702475ADBADD6CEBF9DD3961425780B497A3B909989610A954F23C40'; Install = $false },
        @{ RelativePath = 'NeuralRuntime\dlssnr_amd_pass1.dll'; Size = 56677888; Hash = '430BE589020685D03F0AD92194457BAFE1A186BF658C8CFE2FB7A0BDB7592CC4'; PreviousSize = 12749824; PreviousHash = 'F3D9F2E53B775E4870917572F1F87A28C73068A4DC97252D6FB52360DDF8597A'; Install = $true },
        @{ RelativePath = 'NeuralRuntime\dlssnr_on_amd_weights.bin'; Size = 147689451; Hash = '6BF8DC931EF3CCFFE18C82DE26AB374156E7F19539FFCF8EABAA25DCA5CF15AB'; Install = $true }
    )
    foreach ($asset in $assets) {
        $asset.Path = Join-Path $ReShadeDirectory $asset.RelativePath
        if (-not (Test-Path -LiteralPath $asset.Path -PathType Leaf)) {
            throw "OpenNR package file is missing: $($asset.Path)"
        }
        $item = Get-Item -LiteralPath $asset.Path
        if ($item.Length -ne $asset.Size -or
            (Get-FileHash -LiteralPath $asset.Path -Algorithm SHA256).Hash -ne $asset.Hash) {
            throw "OpenNR package file failed verification: $($asset.Path)"
        }
        if ($asset.Install) {
            $asset.Target = Join-Path $GameDirectory ([IO.Path]::GetFileName($asset.Path))
            if (Test-Path -LiteralPath $asset.Target -PathType Leaf) {
                $targetItem = Get-Item -LiteralPath $asset.Target
                $targetHash = (Get-FileHash -LiteralPath $asset.Target -Algorithm SHA256).Hash
                $matchesCurrent = $targetItem.Length -eq $asset.Size -and $targetHash -eq $asset.Hash
                $matchesPrevious = $null -ne $asset.PreviousHash -and
                    $targetItem.Length -eq $asset.PreviousSize -and $targetHash -eq $asset.PreviousHash
                if (-not $matchesCurrent -and -not $matchesPrevious) {
                    throw "The game directory already contains a different OpenNR runtime file: $($asset.Target)"
                }
                $asset.Replace = $matchesPrevious
            }
        }
    }

    foreach ($asset in $assets | Where-Object { $_.Install }) {
        if ((Test-Path -LiteralPath $asset.Target -PathType Leaf) -and -not $asset.Replace) { continue }
        $temporary = Join-Path $GameDirectory ('.' + [IO.Path]::GetFileName($asset.Target) + '.' + [guid]::NewGuid().ToString('N') + '.tmp')
        try {
            Copy-Item -LiteralPath $asset.Path -Destination $temporary -ErrorAction Stop
            if ((Get-FileHash -LiteralPath $temporary -Algorithm SHA256).Hash -ne $asset.Hash) {
                throw "OpenNR runtime copy failed verification: $($asset.Target)"
            }
            if (Test-Path -LiteralPath $asset.Target -PathType Leaf) {
                Remove-Item -LiteralPath $asset.Target -Force -ErrorAction Stop
            }
            Move-Item -LiteralPath $temporary -Destination $asset.Target -ErrorAction Stop
        }
        finally {
            if (Test-Path -LiteralPath $temporary -PathType Leaf) {
                Remove-Item -LiteralPath $temporary -Force -ErrorAction SilentlyContinue
            }
        }
    }
    Write-Host 'OpenNR ReShade add-on v0.7.14 and AMD runtime v0.6.0 verified.' -ForegroundColor Green
}
