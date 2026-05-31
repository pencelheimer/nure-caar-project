package com.smarttank.app.ui.theme

import android.app.Activity
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat

// ─── light ──────────────────────────────────────────────────────────────────
private val Blue800   = Color(0xFF1565C0)
private val Blue600   = Color(0xFF1E88E5)
private val Cyan500   = Color(0xFF00BCD4)
private val BlueGray50 = Color(0xFFECF3FB)
private val White     = Color(0xFFFFFFFF)
private val OnLight   = Color(0xFF0D1B2A)   // near-black, readable on white/blue
private val OnPrimary = Color(0xFFFFFFFF)

private val LightScheme = lightColorScheme(
    primary              = Blue800,
    onPrimary            = OnPrimary,
    primaryContainer     = Color(0xFFBBDEFB),
    onPrimaryContainer   = Color(0xFF003087),
    secondary            = Cyan500,
    onSecondary          = OnLight,
    secondaryContainer   = Color(0xFFC8E6FF),
    onSecondaryContainer = Color(0xFF001E36),
    tertiary             = Color(0xFF0277BD),
    onTertiary           = White,
    tertiaryContainer    = Color(0xFFB3E5FC),
    onTertiaryContainer  = Color(0xFF00344A),
    background              = BlueGray50,
    onBackground            = OnLight,
    surface                 = White,
    onSurface               = OnLight,
    surfaceTint             = Blue800,
    surfaceVariant          = Color(0xFFDCE8F8),
    onSurfaceVariant        = Color(0xFF3A4A5C),
    surfaceContainer        = Color(0xFFE3EDF7),
    surfaceContainerLow     = Color(0xFFEBF3FB),
    surfaceContainerHigh    = Color(0xFFD8E8F4),
    outline              = Color(0xFF6B8099),
    error                = Color(0xFFB00020),
    onError              = White,
    errorContainer       = Color(0xFFFFDAD6),
    onErrorContainer     = Color(0xFF410002),
)

// ─── dark ────────────────────────────────────────────────────────────────────
private val Navy900   = Color(0xFF0D1B2A)
private val Navy800   = Color(0xFF152233)
private val Blue300   = Color(0xFF90CAF9)
private val Cyan300   = Color(0xFF80DEEA)
private val OnDark    = Color(0xFFE3EEF9)   // light blue-white, readable on dark bg

private val DarkScheme = darkColorScheme(
    primary              = Blue300,
    onPrimary            = Color(0xFF003087),
    primaryContainer     = Blue800,
    onPrimaryContainer   = Color(0xFFD6E8FF),
    secondary            = Cyan300,
    onSecondary          = Color(0xFF003640),
    secondaryContainer   = Color(0xFF004A6E),
    onSecondaryContainer = Color(0xFFC8E6FF),
    tertiary             = Color(0xFF81D4FA),
    onTertiary           = Color(0xFF003548),
    tertiaryContainer    = Color(0xFF004C68),
    onTertiaryContainer  = Color(0xFFB3E5FC),
    background              = Navy900,
    onBackground            = OnDark,
    surface                 = Navy800,
    onSurface               = OnDark,
    surfaceTint             = Blue300,
    surfaceVariant          = Color(0xFF1E3247),
    onSurfaceVariant        = Color(0xFFB0C8E0),
    surfaceContainer        = Color(0xFF192433),
    surfaceContainerLow     = Color(0xFF141D2B),
    surfaceContainerHigh    = Color(0xFF1E2C3D),
    outline              = Color(0xFF5C7A96),
    error                = Color(0xFFFFB4AB),
    onError              = Color(0xFF690005),
    errorContainer       = Color(0xFF93000A),
    onErrorContainer     = Color(0xFFFFDAD6),
)

@Composable
fun SmartTankTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit,
) {
    val view = LocalView.current
    if (!view.isInEditMode) {
        SideEffect {
            val window = (view.context as Activity).window
            // true = dark icons (readable on light background)
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = !darkTheme
        }
    }
    MaterialTheme(
        colorScheme = if (darkTheme) DarkScheme else LightScheme,
        content = content,
    )
}
