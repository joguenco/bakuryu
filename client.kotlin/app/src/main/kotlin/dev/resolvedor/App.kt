package dev.resolvedor

import dev.resolvedor.client.backup.BackupClient
import dev.resolvedor.client.ping.PingClient
import dev.resolvedor.client.version.VersionClient
import io.github.cdimascio.dotenv.dotenv

class App {
    val greeting: String
        get() {
            return "Bakuryu Client"
        }
}

fun main() {
    println(App().greeting)

    val dotenv = dotenv()
    val urlBackupServer = dotenv["URL_BACKUP_SERVER"]
    val tokenSecret = dotenv["TOKEN_SECRET"]

    println(urlBackupServer)
    println(tokenSecret)

    println("Call ping")
    val pingClient = PingClient(urlBackupServer)
    pingClient.ping()

    println("Call version")
    val versionClient = VersionClient(urlBackupServer)
    versionClient.version(tokenSecret)

    println("Call backup")
    val backupClient = BackupClient(urlBackupServer)
    backupClient.backup(
        file = java.io.File("/home/jorgeluis/Backup/odoo/odoo18_angel_2026-01-18_15-42-34.zip"),
        "123",
        tokenSecret = tokenSecret)
}
