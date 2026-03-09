package dev.resolvedor

import dev.resolvedor.client.backup.BackupClient
import dev.resolvedor.client.ping.PingClient
import dev.resolvedor.client.version.VersionClient
import io.github.cdimascio.dotenv.dotenv
import java.io.File
import java.security.MessageDigest

class App {
    val greeting: String
        get() {
            return "Bakuryu Client"
        }
}

fun main(args: Array<String>) {

    val dotenv = dotenv()
    val urlBackupServer = dotenv["URL_SERVER"] ?: error("URL_SERVER not set in .env")
    val tokenSecret = dotenv["TOKEN_SECRET"] ?: error("TOKEN_SECRET not set in .env")

    if (args.isEmpty()) {
        printMessage()
        return
    }

    when (args[0].lowercase()) {
        "-h",
        "--help" -> {
            printMessage()
        }

        "-v",
        "--version" -> {
            println("Bakuryu Client Version: 1.0.0")
        }

        "-p",
        "--ping" -> {
            print("Server responded: ")
            val pingClient = PingClient(urlBackupServer)
            pingClient.ping()
        }

        "-s",
        "--version-server" -> {
            println("Backup Server Version Information:")
            val versionClient = VersionClient(urlBackupServer)
            versionClient.version(tokenSecret)
        }

        "-u",
        "--upload" -> {
            if (args.size < 2) {
                println("Usage: client -u <file-path>")
                return
            }

            val filePath = args[1]
            val file = File(filePath)

            if (!file.exists() || !file.isFile) {
                println("File not found: $filePath")
                return
            }

            val sha2 = getSHA256(file)
            if (sha2!!.isEmpty()) {
                println("File not found: $filePath")
                return
            }

            println("Uploading ...")
            val backupClient = BackupClient(urlBackupServer)
            backupClient.backup(file = file, sha2 = sha2, tokenSecret = tokenSecret)
        }

        else -> {
            println("Unknown command: ${args[0]}")
            println("Write -h or --help to show usage")
        }
    }
}

fun printMessage() {
    println(
        """
        Usage:
        Ping to backup server:
          client --ping
        Show the version number of backup server:
          client --version-server
        Upload backup file:
          client --upload <file>
        Options:
          -h, --help               Show this help message
          -v, --version            Show the version number
          -p, --ping               Ping the backup server
          -s, --version-server     Show the version number of backup server
          -u, --upload <file>      Upload backup file
        """
            .trimIndent()
    )
}

fun getSHA256(file: File): String? {
    if (!file.exists() || !file.isFile) {
        println("File does not exist or is not a file.")
        return ""
    }

    try {
        val digest = MessageDigest.getInstance("SHA-256")
        val inputStream = file.inputStream()
        val byteArray = ByteArray(1024)
        var bytesCount: Int

        while (inputStream.read(byteArray).also { bytesCount = it } != -1) {
            digest.update(byteArray, 0, bytesCount)
        }

        inputStream.close()

        val hashBytes = digest.digest()
        val hexString = StringBuilder()

        for (b in hashBytes) {
            String.format("%02x", b).also { hexString.append(it) }
        }

        return hexString.toString()
    } catch (e: Exception) {
        e.printStackTrace()
        return ""
    }
}
