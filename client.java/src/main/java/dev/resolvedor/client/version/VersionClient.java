package dev.resolvedor.client.version;

import dev.resolvedor.ServiceGenerator;
import retrofit2.Call;
import retrofit2.Response;

public class VersionClient {

    private final ServiceGenerator generator;
    private final String tokenSecret;

    public VersionClient(String urlBackupServer, String tokenSecret) {
        this.generator = new ServiceGenerator(urlBackupServer);
        this.tokenSecret = tokenSecret;
    }

    public void version() {
        VersionService service = generator.createService(VersionService.class, tokenSecret);
        Call<VersionResponse> callSync = service.version();

        try {
            Response<VersionResponse> response = callSync.execute();
            if (response.isSuccessful()) {
                VersionResponse versionResponse = response.body();
                System.out.println(versionResponse.getName());
                System.out.println(versionResponse.getVersion());
                System.out.println(versionResponse.getAuthor());
                System.out.println(versionResponse.getVersionDatabase());
                System.out.println(versionResponse.getVersionOs());
                System.out.println(versionResponse.getVersionRuntime());
            } else {
                System.out.println("Request failed with code: " + response.code());
            }
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }
}