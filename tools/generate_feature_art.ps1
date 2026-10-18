$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

$project = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$source = Join-Path $project 'docs\concepts\earth-limb-source.png'
$image = [System.Drawing.Image]::FromFile($source)
try {
foreach ($theme in @('dark', 'light')) {
    $output = Join-Path $project ('crates\clock_core\assets\earth-limb' + $(if ($theme -eq 'light') { '-light' } else { '' }) + '.png')
    $bitmap = [System.Drawing.Bitmap]::new(600, 200, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $attributes = [System.Drawing.Imaging.ImageAttributes]::new()
    try {
    $graphics.Clear([System.Drawing.Color]::Transparent)
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $matrix = [System.Drawing.Imaging.ColorMatrix]::new()
    $matrix.Matrix33 = if ($theme -eq 'dark') { 0.72 } else { 0.36 }
    if ($theme -eq 'light') {
        # A pale blue photographic treatment keeps the light surface legible.
        $matrix.Matrix00 = 0.4
        $matrix.Matrix11 = 0.4
        $matrix.Matrix22 = 0.3
        $matrix.Matrix40 = 0.3
        $matrix.Matrix41 = 0.5
        $matrix.Matrix42 = 0.7
    }
    $attributes.SetColorMatrix($matrix)
    $photoHeight = [int][Math]::Round(600.0 * $image.Height / $image.Width)
    $target = [System.Drawing.Rectangle]::new(0, -50, 600, $photoHeight)
    $graphics.DrawImage($image, $target, 0, 0, $image.Width, $image.Height,
        [System.Drawing.GraphicsUnit]::Pixel, $attributes)
    $bitmap.Save($output, [System.Drawing.Imaging.ImageFormat]::Png)
    } finally {
        $attributes.Dispose()
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}
} finally {
    $image.Dispose()
}
