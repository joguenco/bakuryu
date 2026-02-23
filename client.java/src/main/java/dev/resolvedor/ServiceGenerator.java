package dev.resolvedor;

import okhttp3.OkHttpClient;
import retrofit2.Retrofit;
import retrofit2.converter.gson.GsonConverterFactory;

public class ServiceGenerator {

    private Retrofit.Builder builder;
    private Retrofit retrofit;
    private OkHttpClient.Builder httpClient;

    public ServiceGenerator(String baseUrl) {
        builder = new Retrofit.Builder().baseUrl(baseUrl).addConverterFactory(GsonConverterFactory.create());
        retrofit = builder.build();
        httpClient = new OkHttpClient.Builder();
    }

    public <S> S createService(Class<S> serviceClass) {
        httpClient.interceptors().clear();
        builder.client(httpClient.build());
        retrofit = builder.build();

        return retrofit.create(serviceClass);
    }
}
