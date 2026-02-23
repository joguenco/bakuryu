package dev.resolvedor.client.version;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class VersionResponse {

    private String name;
    private String author;
    private String version;
    private String versionDatabase;
    private String versionOs;
    private String versionRuntime;
}
