package dev.resolvedor.client

import okhttp3.OkHttpClient
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.io.IOException
import okhttp3.Interceptor
import okhttp3.Request
import okhttp3.Response

class ServiceGenerator(baseUrl: String) {
    private val builder: Retrofit.Builder = Retrofit.Builder().baseUrl(baseUrl).addConverterFactory(GsonConverterFactory.create())
    private var retrofit: Retrofit
    private val httpClient: OkHttpClient.Builder

    init {
        retrofit = builder.build()
        httpClient = OkHttpClient.Builder()
    }

    fun <S> createService(serviceClass: Class<S>): S {
        httpClient.interceptors().clear()
        builder.client(httpClient.build())
        retrofit = builder.build()

        return retrofit.create<S>(serviceClass)
    }

    fun <S> createService(serviceClass: Class<S>, token: String): S {

            httpClient.interceptors().clear()
            httpClient.addInterceptor(object : Interceptor {
                @Throws(IOException::class)
                public override fun intercept(chain: Interceptor.Chain): Response {
                    val original: Request = chain.request()
                    val builder: Request.Builder = original.newBuilder()
                        .header("Accept", "application/json")
                        .header("Authorization", "Bearer " + token)
                    val request = builder.build()
                    return chain.proceed(request)
                }
            })
            builder.client(httpClient.build())
            retrofit = builder.build()

        return retrofit.create<S>(serviceClass)
    }
}
