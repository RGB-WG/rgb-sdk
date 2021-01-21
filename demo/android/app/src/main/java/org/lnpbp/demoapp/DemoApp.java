package org.lnpbp.demoapp;

import android.app.Application;
import android.util.Log;

import org.lnpbp.rgb.Runtime;

import com.getkeepsafe.relinker.ReLinker;

public class DemoApp extends Application {

    private static final String TAG = DemoApp.class.getSimpleName();

    private Runtime runtime;
    public final String network = "testnet";
    public String dataDir;

    @Override
    public void onCreate() {
        super.onCreate();

        final String libName = "rgb_node";
        Log.i(TAG, String.format("Loading '%s' library", libName));
        try {
            ReLinker.loadLibrary(DemoApp.this, "libc++_shared");
            ReLinker.loadLibrary(DemoApp.this, libName);
        } catch (UnsatisfiedLinkError e) {
            Log.e(TAG, String.format("Error loading '%s' library: %s", libName, e.toString()));
        }
        this.dataDir = getFilesDir().toString();

        try {
            this.runtime = new Runtime(this.dataDir, this.network, "pandora.network:60001");
        } catch (Exception e) {
            Log.e(TAG, e.getMessage());
        }
   }

    public Runtime getRuntime() {
        return runtime;
    }
}
