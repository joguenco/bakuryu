package dev.resolvedor.client.version

data class VersionResponse(
    val  name: String,
    val author: String,
    val version: String,
    val versionDatabase: String,
    val versionOs: String,
    val versionRuntime: String
)
