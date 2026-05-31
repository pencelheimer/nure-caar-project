package com.smarttank.app.ui

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import java.io.IOException
import androidx.lifecycle.viewModelScope
import com.google.firebase.messaging.FirebaseMessaging
import com.smarttank.app.SessionManager
import com.smarttank.app.data.api.ApiClient
import com.smarttank.app.data.api.DeviceApiClient
import com.smarttank.app.data.model.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.tasks.await

sealed class DeviceSetupError {
    data object NotReachable : DeviceSetupError()
    data class Other(val cause: String) : DeviceSetupError()
}

class AppViewModel(application: Application) : AndroidViewModel(application) {
    private val session = SessionManager(application)
    private val api = ApiClient.api

    private val _user = MutableStateFlow<UserProfile?>(null)
    val user: StateFlow<UserProfile?> = _user

    private val _reservoirs = MutableStateFlow<List<Reservoir>>(emptyList())
    val reservoirs: StateFlow<List<Reservoir>> = _reservoirs

    private val _devices = MutableStateFlow<List<Device>>(emptyList())
    val devices: StateFlow<List<Device>> = _devices

    private val _measurements = MutableStateFlow<List<Measurement>>(emptyList())
    val measurements: StateFlow<List<Measurement>> = _measurements

    private val _alerts = MutableStateFlow<List<AlertLog>>(emptyList())
    val alerts: StateFlow<List<AlertLog>> = _alerts

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error

    private val _loading = MutableStateFlow(false)
    val loading: StateFlow<Boolean> = _loading

    init {
        if (session.token != null) {
            viewModelScope.launch {
                loadCurrentUser()
                registerFcmToken()
            }
        }
    }

    fun login(email: String, password: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            _loading.value = true
            runCatching { api.login(LoginRequest(email, password)) }
                .onSuccess { auth ->
                    session.token = auth.token
                    loadCurrentUser()
                    registerFcmToken()
                    onSuccess()
                }
                .onFailure { _error.value = it.message }
            _loading.value = false
        }
    }

    fun register(email: String, password: String, firstName: String, lastName: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            _loading.value = true
            runCatching { api.register(RegisterRequest(email, password, firstName, lastName)) }
                .onSuccess { auth ->
                    session.token = auth.token
                    loadCurrentUser()
                    registerFcmToken()
                    onSuccess()
                }
                .onFailure { _error.value = it.message }
            _loading.value = false
        }
    }

    fun logout() {
        session.clear()
        _user.value = null
        _reservoirs.value = emptyList()
        _devices.value = emptyList()
        _measurements.value = emptyList()
        _alerts.value = emptyList()
    }

    fun updateProfile(firstName: String, lastName: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            runCatching { api.updateMe(session.bearerToken, UpdateProfileRequest(firstName, lastName)) }
                .onSuccess { profile ->
                    _user.value = profile
                    onSuccess()
                }
                .onFailure { _error.value = it.message }
        }
    }

    fun changePassword(current: String, new: String, onSuccess: () -> Unit) {
        viewModelScope.launch {
            runCatching { api.changePassword(session.bearerToken, ChangePasswordRequest(current, new)) }
                .onSuccess { onSuccess() }
                .onFailure { _error.value = it.message }
        }
    }

    fun loadReservoirs() {
        viewModelScope.launch {
            runCatching { api.getReservoirs(session.bearerToken) }
                .onSuccess { _reservoirs.value = it }
                .onFailure { _error.value = it.message }
        }
    }

    fun loadDevices() {
        viewModelScope.launch {
            runCatching { api.getDevices(session.bearerToken) }
                .onSuccess { _devices.value = it }
                .onFailure { _error.value = it.message }
        }
    }

    fun loadMeasurements(deviceId: Int) {
        viewModelScope.launch {
            runCatching { api.getMeasurements(session.bearerToken, deviceId) }
                .onSuccess { _measurements.value = it }
                .onFailure { _error.value = it.message }
        }
    }

    fun loadAlerts() {
        viewModelScope.launch {
            runCatching { api.getAlerts(session.bearerToken) }
                .onSuccess { _alerts.value = it }
                .onFailure { _error.value = it.message }
        }
    }

    fun clearError() { _error.value = null }

    fun registerFcmToken() {
        viewModelScope.launch {
            runCatching {
                val token = FirebaseMessaging.getInstance().token.await()
                android.util.Log.i("SmartTankFCM", "Current FCM token: $token")
                api.registerPushToken(session.bearerToken, RegisterPushTokenRequest(token))
            }
        }
    }

    fun fetchDeviceConfig(onSuccess: (DeviceConfig) -> Unit, onError: (DeviceSetupError) -> Unit) {
        viewModelScope.launch {
            runCatching { DeviceApiClient.api.getConfig() }
                .onSuccess { onSuccess(it) }
                .onFailure { e ->
                    onError(
                        if (e is IOException) DeviceSetupError.NotReachable
                        else DeviceSetupError.Other(e.message ?: "Error")
                    )
                }
        }
    }

    fun saveDeviceConfig(config: DeviceConfig, onSuccess: () -> Unit, onError: (String) -> Unit) {
        viewModelScope.launch {
            runCatching { DeviceApiClient.api.setConfig(config) }
                .onSuccess { onSuccess() }
                .onFailure { onError(it.message ?: "Save failed") }
        }
    }

    private suspend fun loadCurrentUser() {
        runCatching { api.getMe(session.bearerToken) }
            .onSuccess { _user.value = it }
            .onFailure { session.clear() }
    }
}
