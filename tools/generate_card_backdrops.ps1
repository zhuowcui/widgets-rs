$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

$root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$assets = Join-Path $root 'crates\clock_core\assets'
New-Item -ItemType Directory -Force -Path $assets | Out-Null

foreach ($spec in @(
    @{ Name = 'small'; Width = 300; Height = 148; CenterX = 350; CenterY = 116; Radius = 145 },
    @{ Name = 'medium'; Width = 300; Height = 306; CenterX = 370; CenterY = 230; Radius = 210 },
    @{ Name = 'large'; Width = 300; Height = 462; CenterX = 390; CenterY = 355; Radius = 255 },
    @{ Name = 'feature'; Width = 850; Height = 250; CenterX = 730; CenterY = 290; Radius = 405 },
    @{ Name = 'app'; Width = 480; Height = 360; CenterX = 424; CenterY = 276; Radius = 284 }
)) {
    $bitmap = [System.Drawing.Bitmap]::new($spec.Width, $spec.Height, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
        $graphics.Clear([System.Drawing.Color]::Transparent)
        if ($spec.Name -eq 'app') {
            $hazePath = [System.Drawing.Drawing2D.GraphicsPath]::new()
            try {
                $hazePath.AddEllipse(16, -132, 472, 312)
                $haze = [System.Drawing.Drawing2D.PathGradientBrush]::new($hazePath)
                try {
                    $haze.CenterColor = [System.Drawing.Color]::FromArgb(26, 41, 105, 187)
                    $haze.SurroundColors = @([System.Drawing.Color]::FromArgb(0, 41, 105, 187))
                    $graphics.FillPath($haze, $hazePath)
                } finally { $haze.Dispose() }
            } finally { $hazePath.Dispose() }
        }
        $wash = [System.Drawing.Drawing2D.LinearGradientBrush]::new(
            [System.Drawing.Rectangle]::new(0, 0, $spec.Width, $spec.Height),
            [System.Drawing.Color]::FromArgb(0, 36, 114, 201),
            [System.Drawing.Color]::FromArgb(22, 36, 114, 201),
            [System.Drawing.Drawing2D.LinearGradientMode]::Horizontal
        )
        try { $graphics.FillRectangle($wash, 0, 0, $spec.Width, $spec.Height) } finally { $wash.Dispose() }

        if ($spec.Name -eq 'medium' -or $spec.Name -eq 'large') {
            $earth = [System.Drawing.Image]::FromFile((Join-Path $root 'docs\concepts\earth-limb-source.png'))
            $attributes = [System.Drawing.Imaging.ImageAttributes]::new()
            try {
                $matrix = [System.Drawing.Imaging.ColorMatrix]::new()
                $matrix.Matrix33 = 0.38
                $attributes.SetColorMatrix($matrix)
                $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
                $target = if ($spec.Name -eq 'medium') {
                    # Extend the source below the card edge; ending it inside the frame
                    # leaves a visible horizontal cut through the planet.
                    [System.Drawing.Rectangle]::new(30, -20, 500, 350)
                } else {
                    [System.Drawing.Rectangle]::new(10, 115, 540, 350)
                }
                $graphics.DrawImage($earth, $target, 0, 0, $earth.Width, $earth.Height,
                    [System.Drawing.GraphicsUnit]::Pixel, $attributes)
            } finally {
                $attributes.Dispose()
                $earth.Dispose()
            }
        }

        if ($spec.Name -ne 'app') {
            $diameter = 2 * $spec.Radius
            $left = $spec.CenterX - $spec.Radius
            $top = $spec.CenterY - $spec.Radius
            foreach ($stroke in @(
                @{ Alpha = 16; Width = 10.0 },
                @{ Alpha = 52; Width = 2.0 },
                @{ Alpha = 24; Width = 1.0 }
            )) {
                $pen = [System.Drawing.Pen]::new([System.Drawing.Color]::FromArgb($stroke.Alpha, 86, 166, 241), [single]$stroke.Width)
                try { $graphics.DrawArc($pen, [single]$left, [single]$top, [single]$diameter, [single]$diameter, 92.0, 176.0) } finally { $pen.Dispose() }
            }
            foreach ($inset in @(36, 82)) {
                $pen = [System.Drawing.Pen]::new([System.Drawing.Color]::FromArgb(16, 96, 168, 237), 1.0)
                try {
                    $graphics.DrawArc($pen, [single]($left + $inset), [single]($top + $inset), [single]($diameter - 2 * $inset), [single]($diameter - 2 * $inset), 94.0, 172.0)
                } finally { $pen.Dispose() }
            }
        }
        if ($spec.Name -eq 'medium' -or $spec.Name -eq 'large') {
            # The widget host reserves a short footer strip outside the Adaptive
            # Card image. Fade the artwork into the host surface before that edge.
            $fadeRows = if ($spec.Name -eq 'medium') { 38 } else { 56 }
            for ($y = $spec.Height - $fadeRows; $y -lt $spec.Height; $y++) {
                $remaining = $spec.Height - $y - 1
                for ($x = 0; $x -lt $spec.Width; $x++) {
                    $pixel = $bitmap.GetPixel($x, $y)
                    if ($pixel.A -eq 0) { continue }
                    $alpha = [int][Math]::Round($pixel.A * $remaining / $fadeRows)
                    $bitmap.SetPixel($x, $y, [System.Drawing.Color]::FromArgb($alpha, $pixel.R, $pixel.G, $pixel.B))
                }
            }
        }
        $path = Join-Path $assets ($spec.Name + '.png')
        $bitmap.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
        if ($spec.Name -ne 'feature' -and $spec.Name -ne 'app') {
            $uri = 'data:image/png;base64,' + [Convert]::ToBase64String([IO.File]::ReadAllBytes($path))
            [IO.File]::WriteAllText((Join-Path $assets ($spec.Name + '.datauri.txt')), $uri)
        }
    } finally {
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}

# Theme-specific card faces cover the card without artificial edge bands.
# The portrait artwork is generated at the intended aspect ratio, never warped.
$portrait = [System.Drawing.Image]::FromFile((Join-Path $root 'docs\concepts\earth-portrait-source-v5.png'))
$jpegCodec = [System.Drawing.Imaging.ImageCodecInfo]::GetImageEncoders() |
    Where-Object MimeType -eq 'image/jpeg' | Select-Object -First 1
try {
    foreach ($spec in @(
        @{ Name = 'small'; Height = 148 },
        @{ Name = 'medium'; Height = 306 },
        @{ Name = 'large'; Height = 462 }
    )) {
        foreach ($theme in @('dark', 'light')) {
            $dark = $theme -eq 'dark'
            $bitmap = [System.Drawing.Bitmap]::new(300, $spec.Height, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
            $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
            try {
                $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
                $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
                $body = if ($dark) { [System.Drawing.Color]::FromArgb(12, 24, 38) } else { [System.Drawing.Color]::FromArgb(237, 244, 252) }
                $gradient = [System.Drawing.Drawing2D.LinearGradientBrush]::new(
                    [System.Drawing.Rectangle]::new(0, 0, 300, $spec.Height), $body, $body,
                    [System.Drawing.Drawing2D.LinearGradientMode]::Vertical)
                try { $graphics.FillRectangle($gradient, 0, 0, 300, $spec.Height) } finally { $gradient.Dispose() }
                if ($spec.Name -ne 'small') {
                    $attributes = [System.Drawing.Imaging.ImageAttributes]::new()
                    try {
                        $matrix = [System.Drawing.Imaging.ColorMatrix]::new()
                        $matrix.Matrix33 = if ($dark) { 0.78 } else { 0.22 }
                        $attributes.SetColorMatrix($matrix)
                        $photoHeight = [int][Math]::Round(300.0 * $portrait.Height / $portrait.Width)
                        $photoTop = if ($spec.Name -eq 'medium') { -100 } else { 0 }
                        $photoLeft = if ($spec.Name -eq 'medium') { 40 } else { 0 }
                        $graphics.DrawImage($portrait, [System.Drawing.Rectangle]::new($photoLeft, $photoTop, 300, $photoHeight),
                            0, 0, $portrait.Width, $portrait.Height, [System.Drawing.GraphicsUnit]::Pixel, $attributes)
                    } finally { $attributes.Dispose() }
                    # Keep labels and their numerals readable across the planet's rim.
                    $shade = [System.Drawing.Drawing2D.LinearGradientBrush]::new(
                        [System.Drawing.Rectangle]::new(0, 0, 300, $spec.Height),
                        [System.Drawing.Color]::FromArgb($(if ($dark) { 150 } else { 76 }), $body.R, $body.G, $body.B),
                        [System.Drawing.Color]::FromArgb(0, $body.R, $body.G, $body.B),
                        [System.Drawing.Drawing2D.LinearGradientMode]::Horizontal)
                    try { $graphics.FillRectangle($shade, 0, 0, 300, $spec.Height) } finally { $shade.Dispose() }
                }
                $name = $spec.Name + '-' + $theme
                $path = Join-Path $assets ($name + '.jpg')
                $parameters = [System.Drawing.Imaging.EncoderParameters]::new(1)
                try {
                    $parameters.Param[0] = [System.Drawing.Imaging.EncoderParameter]::new([System.Drawing.Imaging.Encoder]::Quality, [long]92)
                    $bitmap.Save($path, $jpegCodec, $parameters)
                } finally { $parameters.Dispose() }
                [IO.File]::WriteAllText((Join-Path $assets ($name + '.datauri.txt')),
                    'data:image/jpeg;base64,' + [Convert]::ToBase64String([IO.File]::ReadAllBytes($path)))
            } finally {
                $graphics.Dispose()
                $bitmap.Dispose()
            }
        }
    }
} finally { $portrait.Dispose() }
