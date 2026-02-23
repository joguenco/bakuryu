package dev.resolvedor.client.version

import retrofit2.Call
import retrofit2.http.GET

interface VersionService {
    @GET("/version")
    fun getVersion(): Call<VersionResponse>
}