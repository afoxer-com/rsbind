package com.afoxer.rustlib;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;

import com.afoxer.xxx.ffi.*;


public class MainActivity extends Activity {
    private static final String TAG = "MainActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        LoginService loginService = RustLib.newServices().getLoginService();
        Future future = loginService.login("sidney.wang", "88888888");
        boolean result = future.get();
        Log.i(TAG, "login result is " + result);

        UploadService uploadService = RustLib.newServices().getUploadService();
        uploadService.upload("to/your/path", new UploadProgress() {
            @Override
            public void onProgress(long id, long process, long total) {
                Log.i(TAG, "upload process is " + process);
            }
        });
    }
}
