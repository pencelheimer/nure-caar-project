package com.smarttank.app.ui.screen

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Circle
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.*
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.smarttank.app.R
import com.smarttank.app.data.model.Measurement
import com.smarttank.app.ui.AppViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReservoirDetailScreen(
    reservoirId: Int,
    appViewModel: AppViewModel,
    navController: NavController,
) {
    val reservoirs by appViewModel.reservoirs.collectAsState()
    val devices by appViewModel.devices.collectAsState()
    val measurements by appViewModel.measurements.collectAsState()

    val reservoir = reservoirs.firstOrNull { it.id == reservoirId }
    val device = devices.firstOrNull { it.reservoirId == reservoirId }

    LaunchedEffect(device) {
        device?.let { appViewModel.loadMeasurements(it.id) }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(reservoir?.name ?: stringResource(R.string.reservoir)) },
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
                .padding(16.dp),
        ) {
            if (device != null) {
                val isOnline = device.status == "online"
                val statusColor = if (isOnline) Color(0xFF4CAF50) else Color(0xFF9E9E9E)
                val statusText = if (isOnline) {
                    stringResource(R.string.status_online)
                } else {
                    stringResource(R.string.offline_last_seen, device.lastSeen?.take(16) ?: "–")
                }

                Card(modifier = Modifier.fillMaxWidth()) {
                    Row(modifier = Modifier.padding(16.dp), verticalAlignment = Alignment.CenterVertically) {
                        Icon(Icons.Default.Circle, contentDescription = null,
                            tint = statusColor, modifier = Modifier.size(14.dp))
                        Spacer(Modifier.width(8.dp))
                        Column {
                            Text(device.name, style = MaterialTheme.typography.titleSmall)
                            Text(statusText, style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.outline)
                        }
                    }
                }
                Spacer(Modifier.height(16.dp))
            }

            val latest = measurements.firstOrNull()
            if (latest != null) {
                Card(
                    modifier = Modifier.fillMaxWidth(),
                    colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.primaryContainer),
                ) {
                    Column(
                        modifier = Modifier.padding(20.dp),
                        horizontalAlignment = Alignment.CenterHorizontally,
                    ) {
                        Text(stringResource(R.string.current_level), style = MaterialTheme.typography.labelLarge)
                        Text(
                            stringResource(R.string.liters_format, latest.value),
                            style = MaterialTheme.typography.displaySmall,
                        )
                        reservoir?.let {
                            val pct = (latest.value / it.capacity * 100).coerceIn(0.0, 100.0)
                            Spacer(Modifier.height(8.dp))
                            LinearProgressIndicator(
                                progress = { pct.toFloat() / 100f },
                                modifier = Modifier.fillMaxWidth(),
                            )
                            Text(stringResource(R.string.percent_of_capacity, pct),
                                style = MaterialTheme.typography.bodySmall)
                        }
                    }
                }
                Spacer(Modifier.height(16.dp))
            }

            if (measurements.size >= 2) {
                Text(stringResource(R.string.measurement_history), style = MaterialTheme.typography.titleSmall)
                Spacer(Modifier.height(8.dp))
                MeasurementChart(measurements = measurements,
                    modifier = Modifier.fillMaxWidth().height(180.dp))
            } else if (device == null) {
                Text(stringResource(R.string.no_sensor), color = MaterialTheme.colorScheme.outline)
            } else {
                Text(stringResource(R.string.no_data), color = MaterialTheme.colorScheme.outline)
            }
        }
    }
}

@Composable
private fun MeasurementChart(measurements: List<Measurement>, modifier: Modifier = Modifier) {
    val points = measurements.asReversed()
    val values = points.map { it.value.toFloat() }
    if (values.size < 2) return

    val minVal = values.min()
    val maxVal = values.max()
    val range = (maxVal - minVal).coerceAtLeast(1f)

    val lineColor = MaterialTheme.colorScheme.primary
    val gridColor = MaterialTheme.colorScheme.outlineVariant
    val labelColor = MaterialTheme.colorScheme.outline
    val labelStyle = MaterialTheme.typography.labelSmall

    Column(modifier = modifier) {
        Row(modifier = Modifier.weight(1f)) {
            // Y-axis labels: max / mid / min
            Column(
                modifier = Modifier
                    .width(44.dp)
                    .fillMaxHeight()
                    .padding(end = 6.dp),
                verticalArrangement = Arrangement.SpaceBetween,
                horizontalAlignment = Alignment.End,
            ) {
                Text("${maxVal.toInt()}", style = labelStyle, color = labelColor)
                Text("${((maxVal + minVal) / 2).toInt()}", style = labelStyle, color = labelColor)
                Text("${minVal.toInt()}", style = labelStyle, color = labelColor)
            }

            Canvas(modifier = Modifier.weight(1f).fillMaxHeight()) {
                val w = size.width
                val h = size.height

                fun xFor(i: Int) = i * w / (values.size - 1)
                fun yFor(v: Float) = h - ((v - minVal) / range) * h

                // Dashed horizontal grid lines at 0 %, 50 %, 100 %
                val dashInterval = 6.dp.toPx()
                val dashEffect = PathEffect.dashPathEffect(floatArrayOf(dashInterval, dashInterval))
                listOf(0f, 0.5f, 1f).forEach { frac ->
                    drawLine(
                        color = gridColor.copy(alpha = 0.5f),
                        start = Offset(0f, h - frac * h),
                        end = Offset(w, h - frac * h),
                        strokeWidth = 1.dp.toPx(),
                        pathEffect = dashEffect,
                    )
                }

                val offsets = values.mapIndexed { i, v -> Offset(xFor(i), yFor(v)) }

                // Smooth cubic bezier: control points at the horizontal midpoint
                val linePath = Path().apply {
                    moveTo(offsets[0].x, offsets[0].y)
                    for (i in 0 until offsets.size - 1) {
                        val cpX = (offsets[i].x + offsets[i + 1].x) / 2f
                        cubicTo(cpX, offsets[i].y, cpX, offsets[i + 1].y,
                            offsets[i + 1].x, offsets[i + 1].y)
                    }
                }

                // Gradient area fill under the curve
                val fillPath = Path().apply {
                    addPath(linePath)
                    lineTo(offsets.last().x, h)
                    lineTo(offsets.first().x, h)
                    close()
                }
                drawPath(
                    fillPath,
                    brush = Brush.verticalGradient(
                        colors = listOf(lineColor.copy(alpha = 0.25f), Color.Transparent),
                        startY = 0f, endY = h,
                    ),
                )

                // Line
                drawPath(linePath, color = lineColor,
                    style = Stroke(width = 2.dp.toPx(), cap = StrokeCap.Round, join = StrokeJoin.Round))

                // Dot on the most recent point (rightmost)
                val last = offsets.last()
                drawCircle(color = lineColor, radius = 5.dp.toPx(), center = last)
                drawCircle(color = Color.White, radius = 3.dp.toPx(), center = last)
            }
        }

        // X-axis time labels: first · middle · last
        val times = points.map { it.time.take(16).drop(11) } // "HH:mm"
        Spacer(Modifier.height(4.dp))
        Row(
            modifier = Modifier
                .padding(start = 44.dp)
                .fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(times.first(), style = labelStyle, color = labelColor)
            Text(times[times.size / 2], style = labelStyle, color = labelColor)
            Text(times.last(), style = labelStyle, color = labelColor)
        }
    }
}
