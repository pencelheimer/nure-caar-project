package com.smarttank.app

import android.Manifest
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.ui.res.stringResource
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Notifications
import androidx.compose.material.icons.filled.Person
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.core.content.ContextCompat
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.smarttank.app.ui.AppViewModel
import com.smarttank.app.ui.theme.SmartTankTheme
import kotlinx.coroutines.flow.filterNotNull

class MainActivity : AppCompatActivity() {

    private val requestNotificationPermission =
        registerForActivityResult(ActivityResultContracts.RequestPermission()) { /* no-op */ }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        ThemeManager.init(this)
        createNotificationChannel()
        requestNotificationPermissionIfNeeded()
        applyNotificationIntent(intent)
        enableEdgeToEdge()
        setContent {
            val themeMode by ThemeManager.mode.collectAsState()
            val darkTheme = when (themeMode) {
                ThemeMode.LIGHT -> false
                ThemeMode.DARK -> true
                ThemeMode.SYSTEM -> isSystemInDarkTheme()
            }
            SmartTankTheme(darkTheme = darkTheme) { SmartTankApp() }
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        applyNotificationIntent(intent)
    }

    private fun applyNotificationIntent(intent: Intent?) {
        val id = intent?.getIntExtra("reservoir_id", -1)?.takeIf { it > 0 } ?: return
        NavigationTarget.reservoirId.value = id
    }

    private fun createNotificationChannel() {
        val name = getString(R.string.notification_channel_name)
        val description = getString(R.string.notification_channel_description)
        val channel = NotificationChannel(
            SmartTankMessagingService.CHANNEL_ID,
            name,
            NotificationManager.IMPORTANCE_HIGH,
        ).apply { this.description = description }
        getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
    }

    private fun requestNotificationPermissionIfNeeded() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
            ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS)
            != PackageManager.PERMISSION_GRANTED
        ) {
            requestNotificationPermission.launch(Manifest.permission.POST_NOTIFICATIONS)
        }
    }
}

@Composable
fun SmartTankApp() {
    val appViewModel: AppViewModel = viewModel()
    val user by appViewModel.user.collectAsState()

    if (user == null) {
        val authNav = rememberNavController()
        AuthNavigation(navController = authNav, appViewModel = appViewModel)
        return
    }

    val navController = rememberNavController()
    val backStack by navController.currentBackStackEntryAsState()
    val currentRoute = backStack?.destination?.route

    // Navigate to reservoir when notification is tapped
    LaunchedEffect(Unit) {
        NavigationTarget.reservoirId
            .filterNotNull()
            .collect { id ->
                navController.navigate("reservoir/$id") {
                    popUpTo("dashboard") { inclusive = false }
                }
                NavigationTarget.reservoirId.value = null
            }
    }

    Scaffold(
        bottomBar = {
            NavigationBar {
                NavigationBarItem(
                    selected = currentRoute == "dashboard",
                    onClick = {
                        navController.navigate("dashboard") {
                            launchSingleTop = true
                            popUpTo("dashboard") { inclusive = false }
                        }
                    },
                    icon = { Icon(Icons.Default.Home, contentDescription = null) },
                    label = { Text(stringResource(R.string.reservoirs)) },
                )
                NavigationBarItem(
                    selected = currentRoute == "alerts",
                    onClick = { navController.navigate("alerts") { launchSingleTop = true } },
                    icon = { Icon(Icons.Default.Notifications, contentDescription = null) },
                    label = { Text(stringResource(R.string.alert_log)) },
                )
                NavigationBarItem(
                    selected = currentRoute == "profile",
                    onClick = { navController.navigate("profile") { launchSingleTop = true } },
                    icon = { Icon(Icons.Default.Person, contentDescription = null) },
                    label = { Text(stringResource(R.string.profile)) },
                )
            }
        },
    ) { innerPadding ->
        AppNavigation(
            navController = navController,
            appViewModel = appViewModel,
            modifier = Modifier.padding(innerPadding),
        )
    }
}
