package dev.resolvedor.client.backup

import dev.resolvedor.client.ServiceGenerator
import okhttp3.MediaType.Companion.toMediaTypeOrNull
import okhttp3.MultipartBody
import okhttp3.RequestBody.Companion.asRequestBody
import okhttp3.RequestBody.Companion.toRequestBody
import retrofit2.Call
import java.io.File

class BackupClient(urlBackupServer: String) {

    private val generator: ServiceGenerator = ServiceGenerator(urlBackupServer)

    fun backup(file: File, sha2: String, tokenSecret: String) {
        val service = generator.createService(BackupService::class.java, tokenSecret)

        val sha2Body = sha2.toRequestBody("text/plain".toMediaTypeOrNull())
        val fileRequestBody = file.asRequestBody("application/octet-stream".toMediaTypeOrNull())
        val filePart = MultipartBody.Part.createFormData("file_data", file.name, fileRequestBody)

        try {
            val call: Call<BackupResponse> = service.postBackup(sha2Body, filePart)
            val response = call.execute()
            if (response.isSuccessful) {
                println(response.body()?.message)
            } else {
                println("Request failed with code: ${response.code()}")
            }
        } catch (e: Exception) {
            println(e.message)
        }
    }
}

