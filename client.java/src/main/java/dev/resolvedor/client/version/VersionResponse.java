package dev.resolvedor.client.version;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class VersionResponse {
    
    private String name;
    private String author;
    private String version;
    private String version_database;
    private String version_os;
    private String version_runtime;
}
