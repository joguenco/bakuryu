package dev.resolvedor.client.backup;

import okhttp3.MultipartBody;
import okhttp3.RequestBody;
import okhttp3.ResponseBody;
import retrofit2.Call;
import retrofit2.http.Multipart;
import retrofit2.http.POST;
import retrofit2.http.Part;

public interface BackupService {

    @Multipart
    @POST("/backup")
    Call<ResponseBody> backup(
            @Part("sha2_code") RequestBody sha2Code,
            @Part MultipartBody.Part fileData
    );
}

