$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$assets = Join-Path (Resolve-Path (Join-Path $PSScriptRoot '..')).Path 'packaging\Assets'
New-Item -ItemType Directory -Path $assets -Force | Out-Null

function New-Canvas([int]$width, [int]$height) {
    $image = New-Object System.Drawing.Bitmap($width, $height)
    $graphics = [System.Drawing.Graphics]::FromImage($image)
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $graphics.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit
    return @{ Image = $image; Graphics = $graphics }
}

function Draw-Clock($graphics, [int]$x, [int]$y, [int]$size, [bool]$tile) {
    $blue = [System.Drawing.ColorTranslator]::FromHtml('#3A66F7')
    if ($tile) {
        $tileBrush = New-Object System.Drawing.SolidBrush($blue)
        $graphics.FillRectangle($tileBrush, $x, $y, $size, $size)
        $tileBrush.Dispose()
    }
    $color = if ($tile) { [System.Drawing.Color]::White } else { $blue }
    $stroke = [Math]::Max(2, [int]($size * .055))
    $pen = New-Object System.Drawing.Pen($color, $stroke)
    $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
    $margin = if ($tile) { $size * .23 } else { $size * .1 }
    $radius = $size / 2 - $margin
    $centerX = $x + $size / 2
    $centerY = $y + $size / 2
    $graphics.DrawEllipse($pen, [single]($centerX-$radius), [single]($centerY-$radius), [single](2*$radius), [single](2*$radius))
    $graphics.DrawLine($pen, [single]$centerX, [single]$centerY, [single]$centerX, [single]($centerY-$radius*.6))
    $graphics.DrawLine($pen, [single]$centerX, [single]$centerY, [single]($centerX+$radius*.5), [single]($centerY+$radius*.2))
    $pen.Dispose()
}

function Draw-Hub($graphics, [int]$x, [int]$y, [int]$size) {
    $blue = [System.Drawing.ColorTranslator]::FromHtml('#315BCE')
    $tileBrush = New-Object System.Drawing.SolidBrush($blue)
    $graphics.FillRectangle($tileBrush, $x, $y, $size, $size)
    $white = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
    $pale = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml('#C9D7FF'))
    $margin = [int]($size * .19)
    $gap = [int]([Math]::Max(2, $size * .075))
    $cell = [int](($size - 2*$margin - $gap) / 2)
    foreach ($row in 0..1) { foreach ($col in 0..1) {
        $brush = if ($row -eq 1 -and $col -eq 1) { $pale } else { $white }
        $graphics.FillRectangle($brush, $x + $margin + $col*($cell+$gap), $y + $margin + $row*($cell+$gap), $cell, $cell)
    }}
    $tileBrush.Dispose(); $white.Dispose(); $pale.Dispose()
}

function Save-Icon([string]$name, [int]$size, [bool]$tile) {
    $canvas = New-Canvas $size $size
    if ($name -eq 'ClockIcon.png') { Draw-Clock $canvas.Graphics 0 0 $size $tile }
    else { Draw-Hub $canvas.Graphics 0 0 $size }
    $canvas.Image.Save((Join-Path $assets $name), [System.Drawing.Imaging.ImageFormat]::Png)
    $canvas.Graphics.Dispose(); $canvas.Image.Dispose()
}

Save-Icon 'Square150x150Logo.png' 150 $true
Save-Icon 'Square44x44Logo.png' 44 $true
Save-Icon 'StoreLogo.png' 50 $true
Save-Icon 'ClockIcon.png' 72 $false

$canvas = New-Canvas 310 150
$canvas.Graphics.Clear([System.Drawing.ColorTranslator]::FromHtml('#3A66F7'))
Draw-Hub $canvas.Graphics 18 23 104
$font = New-Object System.Drawing.Font('Segoe UI', 24, [System.Drawing.FontStyle]::Bold)
$canvas.Graphics.DrawString('Widgets RS', $font, [System.Drawing.Brushes]::White, [single]137, [single]51)
$canvas.Image.Save((Join-Path $assets 'Wide310x150Logo.png'), [System.Drawing.Imaging.ImageFormat]::Png)
$font.Dispose(); $canvas.Graphics.Dispose(); $canvas.Image.Dispose()

function Save-Preview([bool]$dark) {
    $canvas = New-Canvas 640 360
    $g = $canvas.Graphics
    $background = if ($dark) { '#131B27' } else { '#EAF0F8' }
    $card = if ($dark) { '#202B3A' } else { '#F8FBFF' }
    $primary = if ($dark) { '#F7F9FD' } else { '#142033' }
    $secondary = if ($dark) { '#AABBD1' } else { '#526780' }
    $g.Clear([System.Drawing.ColorTranslator]::FromHtml($background))
    $cardBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml($card))
    $g.FillRectangle($cardBrush, 16, 12, 608, 336)
    $cardBrush.Dispose()
    $cardBorder = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(42, 105, 158, 220), 2)
    $g.DrawRectangle($cardBorder, 17, 13, 606, 334)
    $cardBorder.Dispose()
    $backdrop = [System.Drawing.Image]::FromFile((Join-Path $PSScriptRoot '..\crates\clock_core\assets\medium.png'))
    try { $g.DrawImage($backdrop, 18, 14, 604, 332) } finally { $backdrop.Dispose() }
    $primaryBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml($primary))
    $secondaryBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml($secondary))
    $small = New-Object System.Drawing.Font('Segoe UI', 13)
    $cue = New-Object System.Drawing.Font('Segoe UI', 17)
    $city = New-Object System.Drawing.Font('Segoe UI', 22, [System.Drawing.FontStyle]::Bold)
    $time = New-Object System.Drawing.Font('Segoe UI', 33, [System.Drawing.FontStyle]::Bold)
    $title = New-Object System.Drawing.Font('Segoe UI', 19, [System.Drawing.FontStyle]::Bold)
    $right = New-Object System.Drawing.StringFormat
    $right.Alignment = [System.Drawing.StringAlignment]::Far
    Draw-Clock $g 42 35 34 $false
    $g.DrawString('World Clock', $title, $primaryBrush, [single]88, [single]37)
    $g.DrawString('Los Angeles', $city, $primaryBrush, [single]44, [single]93)
    $g.DrawString('Today  |  UTC-7', $cue, $secondaryBrush, [single]46, [single]135)
    $g.DrawString('8:29 AM', $time, $primaryBrush, [System.Drawing.RectangleF]::new(330, 99, 255, 66), $right)
    $divider = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(72, 111, 147, 183), 2)
    $g.DrawLine($divider, 45, 181, 587, 181)
    $divider.Dispose()
    $g.DrawString('UTC', $city, $primaryBrush, [single]44, [single]198)
    $g.DrawString('Today  |  UTC+0', $cue, $secondaryBrush, [single]46, [single]240)
    $g.DrawString('3:29 PM', $time, $primaryBrush, [System.Drawing.RectangleF]::new(330, 204, 255, 66), $right)
    $g.DrawString('As of 8:29 AM', $small, $secondaryBrush, [single]46, [single]304)
    $name = if ($dark) { 'ClockPreviewDark.png' } else { 'ClockPreviewLight.png' }
    $canvas.Image.Save((Join-Path $assets $name), [System.Drawing.Imaging.ImageFormat]::Png)
    $small.Dispose(); $cue.Dispose(); $city.Dispose(); $time.Dispose(); $title.Dispose(); $right.Dispose()
    $primaryBrush.Dispose(); $secondaryBrush.Dispose()
    $g.Dispose(); $canvas.Image.Dispose()
}

Save-Preview $false
Save-Preview $true
