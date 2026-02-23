package dev.resolvedor.client.backup

import okhttp3.MultipartBody
import okhttp3.RequestBody
import retrofit2.Call
import retrofit2.http.Multipart
import retrofit2.http.POST
import retrofit2.http.Part

interface BackupService {
    @Multipart
    @POST("/backup")
    fun postBackup(
        @Part("sha2_code") sha2: RequestBody,
        @Part fileData: MultipartBody.Part
    ): Call<BackupResponse>
}