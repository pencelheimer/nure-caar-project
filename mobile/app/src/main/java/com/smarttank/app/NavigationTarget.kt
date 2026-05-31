package com.smarttank.app

import kotlinx.coroutines.flow.MutableStateFlow

/** Carries a pending navigation destination from push notifications to the active NavController. */
object NavigationTarget {
    val reservoirId = MutableStateFlow<Int?>(null)
}
