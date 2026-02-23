package dev.resolvedor;

import java.io.IOException;

import okhttp3.OkHttpClient;
import retrofit2.Retrofit;
import retrofit2.converter.gson.GsonConverterFactory;
import okhttp3.Interceptor;
import okhttp3.Request;
import okhttp3.Response;

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

    public <S> S createService(Class<S> serviceClass, String token) {
        if (token != null) {
            httpClient.interceptors().clear();
            httpClient.addInterceptor(new Interceptor() {
                @Override
                public Response intercept(Interceptor.Chain chain) throws IOException {
                    Request original = chain.request();
                    Request.Builder builder = original.newBuilder()
                            .header("Accept", "application/json")
                            .header("Authorization", "Bearer " + token);
                    Request request = builder.build();
                    return chain.proceed(request);
                }
            });
            builder.client(httpClient.build());
            retrofit = builder.build();
        }
        return retrofit.create(serviceClass);
    }
}
