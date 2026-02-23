package dev.resolvedor.client.ping

import retrofit2.Call
import retrofit2.http.GET


interface PingService {
    @GET("/ping")
    fun getPing(): Call<PingResponse>
}