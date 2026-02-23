package dev.resolvedor.client.backup;

import dev.resolvedor.ServiceGenerator;
import java.io.File;
import okhttp3.MediaType;
import okhttp3.MultipartBody;
import okhttp3.RequestBody;
import okhttp3.ResponseBody;
import retrofit2.Call;
import retrofit2.Response;

public class BackupClient {

    private static final MediaType DEFAULT_FILE_MEDIA_TYPE = MediaType.get("application/octet-stream");

    private final ServiceGenerator generator;
    private final String tokenSecret;

    public BackupClient(String urlBackupServer, String tokenSecret) {
        this.generator = new ServiceGenerator(urlBackupServer);
        this.tokenSecret = tokenSecret;
    }

    public void backup(String sha2Code, File file) {
        BackupService service = generator.createService(BackupService.class, tokenSecret);

        RequestBody sha2CodeBody = RequestBody.create(MediaType.parse(DEFAULT_FILE_MEDIA_TYPE.toString()), sha2Code);
        RequestBody fileBody = RequestBody.create(file, DEFAULT_FILE_MEDIA_TYPE);
        MultipartBody.Part filePart = MultipartBody.Part.createFormData("file_data", file.getName(), fileBody);

        Call<ResponseBody> callSync = service.backup(sha2CodeBody, filePart);

        try {
            Response<ResponseBody> response = callSync.execute();
            if (response.isSuccessful()) {
                ResponseBody body = response.body();
                System.out.println(body != null ? body.string() : "");
            } else {
                String errorBody = response.errorBody() != null ? response.errorBody().string() : "";
                System.out.println("Request failed with code: " + response.code());
                if (!errorBody.isEmpty()) {
                    System.out.println(errorBody);
                }
            }
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }
}
