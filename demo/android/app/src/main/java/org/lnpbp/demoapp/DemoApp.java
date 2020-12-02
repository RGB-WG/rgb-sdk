package org.lnpbp.demoapp;

import android.app.Application;
import android.util.Log;

import org.lnpbp.rgbnode.Runtime;

import java.util.HashMap;

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
            System.loadLibrary(libName);
        } catch (UnsatisfiedLinkError e) {
            Log.e(TAG, String.format("Error loading '%s' library: %s", libName, e.toString()));
        }
        this.dataDir = getFilesDir().toString();

        try {
            this.runtime = new Runtime(this.network, this.dataDir);
        } catch (Exception e) {
            Log.e(TAG, e.getMessage());
        }
   }

    public Runtime getRuntime() {
        return runtime;
    }
}
