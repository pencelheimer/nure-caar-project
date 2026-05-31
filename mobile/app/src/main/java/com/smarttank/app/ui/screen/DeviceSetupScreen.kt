package com.smarttank.app.ui.screen

import android.content.Intent
import android.provider.Settings
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.PhoneAndroid
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.Wifi
import androidx.compose.material.icons.filled.WifiOff
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.smarttank.app.BuildConfig
import com.smarttank.app.R
import com.smarttank.app.data.model.DeviceConfig
import com.smarttank.app.ui.AppViewModel
import com.smarttank.app.ui.DeviceSetupError

private const val TOTAL_STEPS = 3

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSetupScreen(appViewModel: AppViewModel, navController: NavController) {
    var step by remember { mutableStateOf(0) }
    val devices by appViewModel.devices.collectAsState()

    LaunchedEffect(Unit) {
        if (devices.isEmpty()) appViewModel.loadDevices()
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(stringResource(R.string.device_setup)) },
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = null)
                    }
                },
            )
        },
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(horizontal = 24.dp, vertical = 16.dp),
        ) {
            LinearProgressIndicator(
                progress = { (step + 1).toFloat() / TOTAL_STEPS },
                modifier = Modifier.fillMaxWidth(),
            )
            Spacer(Modifier.height(4.dp))
            Text(
                stringResource(R.string.setup_step_of, step + 1, TOTAL_STEPS),
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.outline,
            )
            Spacer(Modifier.height(24.dp))

            when (step) {
                0 -> StepActivate(onNext = { step = 1 })
                1 -> StepConnect(onNext = { step = 2 }, onBack = { step = 0 })
                2 -> StepConfigure(
                    appViewModel = appViewModel,
                    onBack = { step = 1 },
                    onDone = { navController.popBackStack() },
                )
            }
        }
    }
}

@Composable
private fun StepActivate(onNext: () -> Unit) {
    Column(modifier = Modifier.fillMaxSize(), verticalArrangement = Arrangement.SpaceBetween) {
        Column(verticalArrangement = Arrangement.spacedBy(16.dp)) {
            Icon(
                Icons.Default.PhoneAndroid,
                contentDescription = null,
                modifier = Modifier.size(48.dp),
                tint = MaterialTheme.colorScheme.primary,
            )
            Text(stringResource(R.string.setup_step_activate), style = MaterialTheme.typography.headlineSmall)
            Text(stringResource(R.string.setup_activate_desc))
        }
        Button(onClick = onNext, modifier = Modifier.fillMaxWidth()) {
            Text(stringResource(R.string.next))
        }
    }
}

@Composable
private fun StepConnect(onNext: () -> Unit, onBack: () -> Unit) {
    val context = LocalContext.current
    Column(modifier = Modifier.fillMaxSize(), verticalArrangement = Arrangement.SpaceBetween) {
        Column(verticalArrangement = Arrangement.spacedBy(16.dp)) {
            Icon(
                Icons.Default.Wifi,
                contentDescription = null,
                modifier = Modifier.size(48.dp),
                tint = MaterialTheme.colorScheme.primary,
            )
            Text(stringResource(R.string.setup_step_connect), style = MaterialTheme.typography.headlineSmall)
            Text(stringResource(R.string.setup_connect_desc))
            OutlinedButton(
                onClick = { context.startActivity(Intent(Settings.ACTION_WIFI_SETTINGS)) },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Icon(Icons.Default.Settings, contentDescription = null, modifier = Modifier.size(18.dp))
                Spacer(Modifier.width(8.dp))
                Text(stringResource(R.string.setup_open_wifi))
            }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            OutlinedButton(onClick = onBack, modifier = Modifier.weight(1f)) {
                Text(stringResource(R.string.back))
            }
            Button(onClick = onNext, modifier = Modifier.weight(1f)) {
                Text(stringResource(R.string.setup_i_connected))
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun StepConfigure(
    appViewModel: AppViewModel,
    onBack: () -> Unit,
    onDone: () -> Unit,
) {
    val devices by appViewModel.devices.collectAsState()
    var selectedDevice by remember(devices) { mutableStateOf(devices.firstOrNull()) }

    var tankHeight by remember { mutableStateOf("") }
    var tankVolume by remember { mutableStateOf("") }
    var sensorOffset by remember { mutableStateOf("") }
    var wifiSsid by remember { mutableStateOf("") }
    var wifiPass by remember { mutableStateOf("") }
    var serverUrl by remember {
        mutableStateOf("http://${BuildConfig.API_HOST}:${BuildConfig.API_PORT}/devices/measurements")
    }
    var apiKey by remember { mutableStateOf(selectedDevice?.apiKey ?: "") }
    var timeout by remember { mutableStateOf("60000") }

    var fetching by remember { mutableStateOf(true) }
    var fetchError by remember { mutableStateOf<DeviceSetupError?>(null) }
    var saving by remember { mutableStateOf(false) }
    var saveError by remember { mutableStateOf<String?>(null) }
    var done by remember { mutableStateOf(false) }
    var deviceMenuExpanded by remember { mutableStateOf(false) }

    fun applyConfig(c: DeviceConfig) {
        tankHeight = c.tankHeightMm.toString()
        tankVolume = c.tankVolumeL.toString()
        sensorOffset = c.sensorOffsetMm.toString()
        wifiSsid = c.wifiSsid
        wifiPass = c.wifiPass
        serverUrl = c.serverUrl
        apiKey = c.apiKey
        timeout = c.timeout.toString()
    }

    fun doFetch() {
        fetching = true
        fetchError = null
        appViewModel.fetchDeviceConfig(
            onSuccess = { c -> applyConfig(c); fetching = false },
            onError = { e -> fetchError = e; fetching = false },
        )
    }

    LaunchedEffect(Unit) { doFetch() }
    LaunchedEffect(selectedDevice) { selectedDevice?.let { apiKey = it.apiKey } }

    if (done) {
        Column(
            modifier = Modifier.fillMaxSize(),
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Icon(
                Icons.Default.CheckCircle,
                contentDescription = null,
                modifier = Modifier.size(72.dp),
                tint = MaterialTheme.colorScheme.primary,
            )
            Spacer(Modifier.height(16.dp))
            Text(stringResource(R.string.setup_success_title), style = MaterialTheme.typography.titleLarge)
            Spacer(Modifier.height(8.dp))
            Text(
                stringResource(R.string.setup_success_desc),
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.outline,
            )
            Spacer(Modifier.height(32.dp))
            Button(onClick = onDone, modifier = Modifier.fillMaxWidth()) {
                Text(stringResource(R.string.done))
            }
        }
        return
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState()),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(stringResource(R.string.setup_step_configure), style = MaterialTheme.typography.headlineSmall)

        when {
            fetching -> Box(Modifier.fillMaxWidth(), contentAlignment = Alignment.Center) {
                CircularProgressIndicator()
            }
            fetchError is DeviceSetupError.NotReachable -> {
                Card(
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.secondaryContainer,
                    ),
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Row(
                            verticalAlignment = Alignment.CenterVertically,
                            horizontalArrangement = Arrangement.spacedBy(8.dp),
                        ) {
                            Icon(
                                Icons.Default.WifiOff,
                                contentDescription = null,
                                tint = MaterialTheme.colorScheme.onSecondaryContainer,
                            )
                            Text(
                                stringResource(R.string.setup_not_reachable_title),
                                style = MaterialTheme.typography.titleSmall,
                                color = MaterialTheme.colorScheme.onSecondaryContainer,
                            )
                        }
                        Text(
                            stringResource(R.string.setup_not_reachable_desc),
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSecondaryContainer,
                        )
                        OutlinedButton(onClick = ::doFetch, modifier = Modifier.fillMaxWidth()) {
                            Text(stringResource(R.string.retry))
                        }
                    }
                }
            }
            fetchError is DeviceSetupError.Other -> Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                Text(
                    stringResource(R.string.setup_fetch_error, (fetchError as DeviceSetupError.Other).cause),
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodySmall,
                )
                OutlinedButton(onClick = ::doFetch, modifier = Modifier.fillMaxWidth()) {
                    Text(stringResource(R.string.retry))
                }
            }
            else -> Unit
        }

        if (!fetching) {
            // Device picker for api_key auto-fill
            if (devices.isNotEmpty()) {
                ExposedDropdownMenuBox(
                    expanded = deviceMenuExpanded,
                    onExpandedChange = { deviceMenuExpanded = it },
                ) {
                    OutlinedTextField(
                        value = selectedDevice?.name ?: stringResource(R.string.setup_select_device),
                        onValueChange = {},
                        readOnly = true,
                        label = { Text(stringResource(R.string.setup_device_label)) },
                        trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(deviceMenuExpanded) },
                        modifier = Modifier
                            .fillMaxWidth()
                            .menuAnchor(MenuAnchorType.PrimaryNotEditable),
                    )
                    ExposedDropdownMenu(
                        expanded = deviceMenuExpanded,
                        onDismissRequest = { deviceMenuExpanded = false },
                    ) {
                        devices.forEach { device ->
                            DropdownMenuItem(
                                text = { Text(device.name) },
                                onClick = { selectedDevice = device; deviceMenuExpanded = false },
                            )
                        }
                    }
                }
            }

            HorizontalDivider()

            val decimalKeyboard = KeyboardOptions(keyboardType = KeyboardType.Decimal)
            OutlinedTextField(
                tankHeight, { tankHeight = it },
                label = { Text(stringResource(R.string.tank_height_mm)) },
                keyboardOptions = decimalKeyboard, singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                tankVolume, { tankVolume = it },
                label = { Text(stringResource(R.string.tank_volume_l)) },
                keyboardOptions = decimalKeyboard, singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                sensorOffset, { sensorOffset = it },
                label = { Text(stringResource(R.string.sensor_offset_mm)) },
                keyboardOptions = decimalKeyboard, singleLine = true,
                modifier = Modifier.fillMaxWidth(),
            )

            HorizontalDivider()

            OutlinedTextField(
                wifiSsid, { wifiSsid = it },
                label = { Text(stringResource(R.string.wifi_ssid)) },
                singleLine = true, modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                wifiPass, { wifiPass = it },
                label = { Text(stringResource(R.string.wifi_password)) },
                singleLine = true, modifier = Modifier.fillMaxWidth(),
            )

            HorizontalDivider()

            OutlinedTextField(
                serverUrl, { serverUrl = it },
                label = { Text(stringResource(R.string.server_url)) },
                singleLine = true, modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                apiKey, { apiKey = it },
                label = { Text(stringResource(R.string.api_key_label)) },
                singleLine = true, modifier = Modifier.fillMaxWidth(),
            )
            OutlinedTextField(
                timeout, { timeout = it },
                label = { Text(stringResource(R.string.measurement_interval_ms)) },
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                singleLine = true, modifier = Modifier.fillMaxWidth(),
            )

            saveError?.let {
                Text(it, color = MaterialTheme.colorScheme.error, style = MaterialTheme.typography.bodySmall)
            }

            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                OutlinedButton(onClick = onBack, modifier = Modifier.weight(1f)) {
                    Text(stringResource(R.string.back))
                }
                Button(
                    onClick = {
                        saving = true
                        saveError = null
                        appViewModel.saveDeviceConfig(
                            DeviceConfig(
                                tankHeightMm = tankHeight.toFloatOrNull() ?: 0f,
                                tankVolumeL = tankVolume.toFloatOrNull() ?: 0f,
                                sensorOffsetMm = sensorOffset.toFloatOrNull() ?: 0f,
                                wifiSsid = wifiSsid,
                                wifiPass = wifiPass,
                                serverUrl = serverUrl,
                                apiKey = apiKey,
                                timeout = timeout.toIntOrNull() ?: 60000,
                            ),
                            onSuccess = { saving = false; done = true },
                            onError = { e -> saving = false; saveError = e },
                        )
                    },
                    enabled = !saving,
                    modifier = Modifier.weight(1f),
                ) {
                    if (saving) CircularProgressIndicator(
                        Modifier.size(20.dp), strokeWidth = 2.dp,
                        color = MaterialTheme.colorScheme.onPrimary,
                    )
                    else Text(stringResource(R.string.save_to_device))
                }
            }

            Spacer(Modifier.height(16.dp))
        }
    }
}
