package dev.resolvedor.client.ping

import dev.resolvedor.client.ServiceGenerator
import retrofit2.Call
import retrofit2.Response

class PingClient(urlBackupServer: String) {
    private val generator: ServiceGenerator = ServiceGenerator(urlBackupServer)

    fun ping() {
        val service = generator.createService(serviceClass = PingService::class.java)
        val call = service.getPing()

        try {
            val response = call.execute()
            if (response.isSuccessful) {
                val pingResponse = response.body()
                println(pingResponse?.message)
            } else {
                println("Request failed with code: " + response.code())
            }
        } catch (e: Exception) {
            println(e.message)
        }
    }
}
