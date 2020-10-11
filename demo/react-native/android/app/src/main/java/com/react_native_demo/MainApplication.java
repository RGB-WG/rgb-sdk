package com.react_native_demo;

import android.app.Application;
import android.util.Log;

import com.facebook.react.PackageList;
import com.facebook.react.ReactApplication;
import com.facebook.react.ReactNativeHost;
import com.facebook.react.ReactPackage;
import com.facebook.soloader.SoLoader;
import java.util.HashMap;
import java.util.List;

import org.lnpbp.rgbnode.Runtime;

public class MainApplication extends Application implements ReactApplication {
    private Runtime runtime;

  private final ReactNativeHost mReactNativeHost =
      new ReactNativeHost(this) {
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

      Log.i("RGB_NODE", "loading library");
      System.loadLibrary("rgb_node");

      final String datadir = getFilesDir().toString();
      final String network = "testnet";

      final HashMap contractEndpoints = new HashMap();
      contractEndpoints.put("Fungible", String.format("inproc://fungibled", datadir, network));
      this.runtime = new Runtime(network, String.format("inproc://stashd", datadir, network), contractEndpoints, true, datadir);
  }
}
