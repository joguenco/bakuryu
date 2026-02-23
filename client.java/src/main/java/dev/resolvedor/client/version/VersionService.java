package dev.resolvedor.client.version;

import retrofit2.Call;
import retrofit2.http.GET;

public interface VersionService {
    @GET("/version")
    public Call<VersionResponse> version();
}
