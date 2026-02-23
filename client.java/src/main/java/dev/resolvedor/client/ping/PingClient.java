package dev.resolvedor.client.ping;

import dev.resolvedor.ServiceGenerator;
import retrofit2.Call;
import retrofit2.Response;

public class PingClient {

    private final ServiceGenerator generator;

    public PingClient(String urlBackupServer) {
        this.generator = new ServiceGenerator(urlBackupServer);
    }

    public void ping() {
        PingService service = generator.createService(PingService.class);
        Call<PingResponse> callSync = service.ping();

        try {
            Response<PingResponse> response = callSync.execute();
            if (response.isSuccessful()) {
                PingResponse pingResponse = response.body();
                System.out.println(pingResponse.getMessage());
            } else {
                System.out.println("Request failed with code: " + response.code());
            }
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }

}
