package com.smarttank.app.ui.screen

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Circle
import androidx.compose.material.icons.filled.Router
import androidx.compose.material.icons.filled.WaterDrop
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.smarttank.app.R
import com.smarttank.app.data.model.Device
import com.smarttank.app.data.model.Reservoir
import com.smarttank.app.ui.AppViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DashboardScreen(
    appViewModel: AppViewModel,
    navController: NavController,
) {
    val reservoirs by appViewModel.reservoirs.collectAsState()
    val devices by appViewModel.devices.collectAsState()

    LaunchedEffect(Unit) {
        appViewModel.loadReservoirs()
        appViewModel.loadDevices()
    }

    Scaffold(
        topBar = { TopAppBar(title = { Text(stringResource(R.string.reservoirs)) }) },
        floatingActionButton = {
            FloatingActionButton(onClick = { navController.navigate("device_setup") }) {
                Icon(Icons.Default.Router, contentDescription = stringResource(R.string.device_setup))
            }
        },
    ) { padding ->
        if (reservoirs.isEmpty()) {
            Box(Modifier.fillMaxSize().padding(padding), contentAlignment = Alignment.Center) {
                Text(stringResource(R.string.no_reservoirs), color = MaterialTheme.colorScheme.outline)
            }
        } else {
            LazyColumn(
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp),
                modifier = Modifier.padding(padding),
            ) {
                items(reservoirs) { reservoir ->
                    val device = devices.firstOrNull { it.reservoirId == reservoir.id }
                    ReservoirCard(
                        reservoir = reservoir,
                        device = device,
                        onClick = { navController.navigate("reservoir/${reservoir.id}") },
                    )
                }
            }
        }
    }
}

@Composable
private fun ReservoirCard(
    reservoir: Reservoir,
    device: Device?,
    onClick: () -> Unit,
) {
    val isOnline = device?.status == "online"
    val statusLabel = stringResource(if (isOnline) R.string.status_online else R.string.status_offline)
    val statusColor = if (isOnline) Color(0xFF4CAF50) else Color(0xFF9E9E9E)

    Card(
        modifier = Modifier.fillMaxWidth().clickable(onClick = onClick),
        elevation = CardDefaults.cardElevation(2.dp),
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Icon(
                Icons.Default.WaterDrop,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.primary,
                modifier = Modifier.size(40.dp),
            )
            Spacer(Modifier.width(16.dp))
            Column(Modifier.weight(1f)) {
                Text(reservoir.name, style = MaterialTheme.typography.titleMedium)
                reservoir.location?.let {
                    Text(it, style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.outline)
                }
                Text(
                    stringResource(R.string.capacity_format, reservoir.capacity.toInt()),
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            if (device != null) {
                Column(horizontalAlignment = Alignment.End) {
                    Icon(Icons.Default.Circle, contentDescription = null,
                        tint = statusColor, modifier = Modifier.size(12.dp))
                    Spacer(Modifier.height(4.dp))
                    Text(statusLabel, style = MaterialTheme.typography.labelSmall, color = statusColor)
                }
            }
        }
    }
}
