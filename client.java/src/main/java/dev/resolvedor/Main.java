package dev.resolvedor;

import dev.resolvedor.client.backup.BackupClient;
import dev.resolvedor.client.ping.PingClient;
import dev.resolvedor.client.version.VersionClient;
import io.github.cdimascio.dotenv.Dotenv;

import java.io.File;

public class Main {
    public static void main(String[] args) {

        System.out.println("Bakuryu Client");

        Dotenv dotenv = Dotenv.load();
        String urlBackupServer = dotenv.get("URL_BACKUP_SERVER");
        String tokenSecret = dotenv.get("TOKEN_SECRET");

        System.out.println(urlBackupServer);
        System.out.println(tokenSecret);

        PingClient pingClient = new PingClient(urlBackupServer);
        pingClient.ping();

        VersionClient versionClient = new VersionClient(urlBackupServer, tokenSecret);
        versionClient.version();

        BackupClient backupClient = new BackupClient(urlBackupServer, tokenSecret);
        
        File file = new File("/data/backup/odoo/ovm/ovm18_2025-08-05_15-47-02.zip");
        backupClient.backup("123456", file);
    }
}