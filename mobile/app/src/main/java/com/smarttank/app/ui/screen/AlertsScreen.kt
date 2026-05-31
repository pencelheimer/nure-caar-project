package com.smarttank.app.ui.screen

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Warning
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.smarttank.app.R
import com.smarttank.app.data.model.AlertLog
import com.smarttank.app.data.model.Reservoir
import com.smarttank.app.ui.AppViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AlertsScreen(appViewModel: AppViewModel, navController: NavController) {
    val alerts by appViewModel.alerts.collectAsState()
    val reservoirs by appViewModel.reservoirs.collectAsState()

    LaunchedEffect(Unit) {
        appViewModel.loadAlerts()
        appViewModel.loadReservoirs()
    }

    Scaffold(
        topBar = { TopAppBar(title = { Text(stringResource(R.string.alert_log)) }) },
    ) { padding ->
        if (alerts.isEmpty()) {
            Box(Modifier.fillMaxSize().padding(padding), contentAlignment = Alignment.Center) {
                Text(stringResource(R.string.no_alerts), color = MaterialTheme.colorScheme.outline)
            }
        } else {
            LazyColumn(
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(10.dp),
                modifier = Modifier.padding(padding),
            ) {
                items(alerts) { alert ->
                    val reservoir = reservoirs.firstOrNull { it.id == alert.reservoirId }
                    AlertCard(
                        alert = alert,
                        reservoir = reservoir,
                        onClick = { navController.navigate("reservoir/${alert.reservoirId}") },
                    )
                }
            }
        }
    }
}

@Composable
private fun AlertCard(alert: AlertLog, reservoir: Reservoir?, onClick: () -> Unit) {
    val conditionLabel = when (alert.conditionType) {
        "less_than"    -> stringResource(R.string.condition_below)
        "greater_than" -> stringResource(R.string.condition_above)
        else           -> alert.conditionType
    }

    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.errorContainer),
    ) {
        Row(modifier = Modifier.padding(14.dp), verticalAlignment = Alignment.CenterVertically) {
            Icon(
                Icons.Default.Warning,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.error,
                modifier = Modifier.size(28.dp),
            )
            Spacer(Modifier.width(12.dp))
            Column {
                if (reservoir != null) {
                    Text(
                        reservoir.name,
                        style = MaterialTheme.typography.titleSmall,
                    )
                }
                Text(
                    stringResource(R.string.alert_level_condition, conditionLabel, alert.threshold.toInt()),
                    style = MaterialTheme.typography.bodyMedium,
                )
                Text(
                    stringResource(R.string.alert_value_format, alert.value),
                    style = MaterialTheme.typography.bodySmall,
                )
                Text(
                    alert.triggeredAt.take(16).replace("T", " "),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.outline,
                )
            }
        }
    }
}
