package dev.resolvedor.client.version

import dev.resolvedor.client.ServiceGenerator

class VersionClient(urlBackupServer: String)  {

    private val generator: ServiceGenerator = ServiceGenerator(urlBackupServer)

    fun version(tokenSecret: String) {
        val service = generator.createService(serviceClass = VersionService::class.java,
            token = tokenSecret)
        val call = service.getVersion()

        try {
            val response = call.execute()
            if (response.isSuccessful) {
                val versionResponse = response.body()
                println(versionResponse?.name)
                println(versionResponse?.version)
                println(versionResponse?.author)
                println(versionResponse?.versionOs)
                println(versionResponse?.versionDatabase)
                println(versionResponse?.versionRuntime)
            }
        } catch (e: Exception) {
            println(e.message)
        }
    }
}