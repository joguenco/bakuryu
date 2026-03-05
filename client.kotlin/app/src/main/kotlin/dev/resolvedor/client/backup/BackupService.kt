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
        @Part("sha2") sha2: RequestBody,
        @Part file: MultipartBody.Part,
    ): Call<BackupResponse>
}
