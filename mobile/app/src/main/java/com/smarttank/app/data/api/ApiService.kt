package com.smarttank.app.data.api

import com.smarttank.app.data.model.*
import retrofit2.http.*

interface ApiService {

    // Auth
    @POST("auth/login")
    suspend fun login(@Body request: LoginRequest): AuthResponse

    @POST("auth/register")
    suspend fun register(@Body request: RegisterRequest): AuthResponse

    @GET("auth/me")
    suspend fun getMe(@Header("Authorization") token: String): UserProfile

    @PUT("auth/me")
    suspend fun updateMe(
        @Header("Authorization") token: String,
        @Body request: UpdateProfileRequest,
    ): UserProfile

    @POST("auth/change-password")
    suspend fun changePassword(
        @Header("Authorization") token: String,
        @Body request: ChangePasswordRequest,
    )

    // Reservoirs
    @GET("reservoirs")
    suspend fun getReservoirs(@Header("Authorization") token: String): List<Reservoir>

    @GET("reservoirs/{id}")
    suspend fun getReservoir(
        @Header("Authorization") token: String,
        @Path("id") id: Int,
    ): Reservoir

    // Devices
    @GET("devices")
    suspend fun getDevices(@Header("Authorization") token: String): List<Device>

    // Measurements
    @GET("devices/{id}/measurements")
    suspend fun getMeasurements(
        @Header("Authorization") token: String,
        @Path("id") deviceId: Int,
        @Query("limit") limit: Int = 100,
    ): List<Measurement>

    // Alerts
    @GET("alerts")
    suspend fun getAlerts(@Header("Authorization") token: String): List<AlertLog>

    // Push tokens
    @POST("users/push-token")
    suspend fun registerPushToken(
        @Header("Authorization") token: String,
        @Body request: RegisterPushTokenRequest,
    )
}
