package dev.resolvedor.client.ping;

import retrofit2.Call;
import retrofit2.http.GET;

public interface PingService {
    @GET("/ping")
    public Call<PingResponse> ping();
}
