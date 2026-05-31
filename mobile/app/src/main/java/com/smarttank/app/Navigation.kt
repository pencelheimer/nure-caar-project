package com.smarttank.app

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.smarttank.app.ui.AppViewModel
import com.smarttank.app.ui.screen.*

@Composable
fun AppNavigation(
    navController: NavHostController,
    appViewModel: AppViewModel,
    modifier: Modifier = Modifier,
) {
    NavHost(navController = navController, startDestination = "dashboard", modifier = modifier) {
        composable("dashboard") {
            DashboardScreen(appViewModel = appViewModel, navController = navController)
        }
        composable(
            "reservoir/{id}",
            arguments = listOf(navArgument("id") { type = NavType.IntType }),
        ) { back ->
            ReservoirDetailScreen(
                reservoirId = back.arguments!!.getInt("id"),
                appViewModel = appViewModel,
                navController = navController,
            )
        }
        composable("alerts") {
            AlertsScreen(appViewModel = appViewModel, navController = navController)
        }
        composable("profile") {
            ProfileScreen(appViewModel = appViewModel)
        }
        composable("device_setup") {
            DeviceSetupScreen(appViewModel = appViewModel, navController = navController)
        }
    }
}

@Composable
fun AuthNavigation(
    navController: NavHostController,
    appViewModel: AppViewModel,
) {
    NavHost(navController = navController, startDestination = "login") {
        composable("login") {
            LoginScreen(
                appViewModel = appViewModel,
                onNavigateToRegister = { navController.navigate("register") },
            )
        }
        composable("register") {
            RegisterScreen(
                appViewModel = appViewModel,
                onNavigateToLogin = { navController.popBackStack() },
            )
        }
    }
}
