package com.smarttank.app.data.api

import com.smarttank.app.data.model.DeviceConfig
import okhttp3.OkHttpClient
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.POST
import java.util.concurrent.TimeUnit

interface DeviceApiService {
    @GET("config")
    suspend fun getConfig(): DeviceConfig

    @POST("config")
    suspend fun setConfig(@Body config: DeviceConfig): String
}

object DeviceApiClient {
    val api: DeviceApiService = Retrofit.Builder()
        .baseUrl("http://192.168.4.1/")
        .addConverterFactory(GsonConverterFactory.create())
        .client(
            OkHttpClient.Builder()
                .connectTimeout(5, TimeUnit.SECONDS)
                .readTimeout(5, TimeUnit.SECONDS)
                .writeTimeout(5, TimeUnit.SECONDS)
                .build()
        )
        .build()
        .create(DeviceApiService::class.java)
}
