package com.smarttank.app.data.model

import com.google.gson.annotations.SerializedName

data class AuthResponse(
    val token: String,
    @SerializedName("user_id") val userId: Int,
    val email: String,
)

data class LoginRequest(val email: String, val password: String)

data class RegisterRequest(
    val email: String,
    val password: String,
    @SerializedName("first_name") val firstName: String,
    @SerializedName("last_name") val lastName: String,
)

data class UserProfile(
    val id: Int,
    val email: String,
    @SerializedName("first_name") val firstName: String,
    @SerializedName("last_name") val lastName: String,
    val role: String,
    @SerializedName("created_at") val createdAt: String,
)

data class UpdateProfileRequest(
    @SerializedName("first_name") val firstName: String,
    @SerializedName("last_name") val lastName: String,
)

data class ChangePasswordRequest(
    @SerializedName("current_password") val currentPassword: String,
    @SerializedName("new_password") val newPassword: String,
)

data class Reservoir(
    val id: Int,
    val name: String,
    val description: String?,
    val capacity: Double,
    val location: String?,
)

data class Device(
    val id: Int,
    val name: String,
    @SerializedName("reservoir_id") val reservoirId: Int?,
    val status: String,
    @SerializedName("last_seen") val lastSeen: String?,
    @SerializedName("api_key") val apiKey: String,
)

data class Measurement(
    val time: String,
    val value: Double,
    @SerializedName("device_id") val deviceId: Int,
)

data class DeviceConfig(
    @SerializedName("tank_height_mm") val tankHeightMm: Float,
    @SerializedName("tank_volume_l") val tankVolumeL: Float,
    @SerializedName("sensor_offset_mm") val sensorOffsetMm: Float,
    @SerializedName("wifi_ssid") val wifiSsid: String,
    @SerializedName("wifi_pass") val wifiPass: String,
    @SerializedName("server_url") val serverUrl: String,
    @SerializedName("api_key") val apiKey: String,
    val timeout: Int,
)

data class RegisterPushTokenRequest(
    val token: String,
    val platform: String = "android",
)

data class AlertLog(
    val id: Int,
    @SerializedName("rule_id") val ruleId: Int,
    @SerializedName("reservoir_id") val reservoirId: Int,
    @SerializedName("triggered_at") val triggeredAt: String,
    val status: String,
    @SerializedName("condition_type") val conditionType: String,
    val threshold: Double,
    val value: Double,
)
