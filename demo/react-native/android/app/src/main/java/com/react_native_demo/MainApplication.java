package com.react_native_demo;

import android.app.Application;
import android.util.Log;

import com.facebook.react.PackageList;
import com.facebook.react.ReactApplication;
import com.facebook.react.ReactNativeHost;
import com.facebook.react.ReactPackage;
import com.facebook.soloader.SoLoader;
import java.util.List;

import org.lnpbp.rgbnode.Runtime;

public class MainApplication extends Application implements ReactApplication {

    private Runtime runtime;
    public final String network = "testnet";
    public String dataDir;

    private static final String TAG = MainApplication.class.getSimpleName();

    private final ReactNativeHost mReactNativeHost = new ReactNativeHost(this) {
        @Override
        public boolean getUseDeveloperSupport() {
            return BuildConfig.DEBUG;
        }

        @Override
        protected List<ReactPackage> getPackages() {
            @SuppressWarnings("UnnecessaryLocalVariable")
            List<ReactPackage> packages = new PackageList(this).getPackages();
            // Packages that cannot be autolinked yet can be added manually here, for example:
            // packages.add(new MyReactNativePackage());
            packages.add(new DemoPackage());
            return packages;
        }

        @Override
        protected String getJSMainModuleName() {
            return "index";
        }
    };

    public Runtime getRuntime() {
        return runtime;
    }

    @Override
    public ReactNativeHost getReactNativeHost() {
        return mReactNativeHost;
    }

    @Override
    public void onCreate() {
        super.onCreate();
        SoLoader.init(this, /* native exopackage */ false);

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
}
