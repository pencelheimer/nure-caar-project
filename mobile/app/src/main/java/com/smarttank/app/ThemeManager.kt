package com.smarttank.app

import android.content.Context
import android.content.SharedPreferences
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

enum class ThemeMode { SYSTEM, LIGHT, DARK }

object ThemeManager {
    private lateinit var prefs: SharedPreferences
    private val _mode = MutableStateFlow(ThemeMode.SYSTEM)
    val mode: StateFlow<ThemeMode> = _mode

    fun init(context: Context) {
        prefs = context.getSharedPreferences("theme_prefs", Context.MODE_PRIVATE)
        _mode.value = ThemeMode.valueOf(
            prefs.getString("theme_mode", ThemeMode.SYSTEM.name) ?: ThemeMode.SYSTEM.name
        )
    }

    fun setMode(mode: ThemeMode) {
        _mode.value = mode
        prefs.edit().putString("theme_mode", mode.name).apply()
    }
}
