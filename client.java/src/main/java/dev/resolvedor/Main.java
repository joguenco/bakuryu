package dev.resolvedor;

import dev.resolvedor.client.ping.PingResponse;
import dev.resolvedor.client.ping.PingService;
import retrofit2.Call;
import retrofit2.Response;

public class Main {
    public static void main(String[] args) {

        System.out.println("Bakuryu Client");
        ServiceGenerator generator = new ServiceGenerator("https://bakuryu.resolvedor.dev");
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