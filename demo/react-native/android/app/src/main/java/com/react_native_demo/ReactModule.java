package com.react_native_demo;

import androidx.annotation.NonNull;

import com.facebook.react.bridge.Arguments;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.WritableMap;

import org.lnpbp.rgbnode.Runtime;
import org.lnpbp.rgbnode.model.IssueArgs;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;

public class ReactModule extends ReactContextBaseJavaModule {
    public ReactModule(ReactApplicationContext reactContext) {
        super(reactContext);
    }

    @NonNull
    @Override
    public String getName() {
        return "RGB";
    }

    @ReactMethod
    public void issue( // TODO: alternatively we can just take a json string here and pass that directly to the ffi for librgb
            int alloc_coins,
            String alloc_outpoint,

            String network,
            String ticker,
            String name,
            String description,
            int precision,

            Promise promise) {
        try {
            final Runtime runtime = ((MainApplication) getCurrentActivity().getApplication()).getRuntime();

            final IssueArgs.OutpointCoins allocation = new IssueArgs.OutpointCoins((long) alloc_coins, alloc_outpoint);
            runtime.issue(network, ticker, name, description, precision, Arrays.asList(allocation), new HashSet<IssueArgs.OutpointCoins>(), null, null);


            WritableMap map = Arguments.createMap();
            promise.resolve(map);
        } catch (Exception e) {
            promise.reject(e);
        }
    }
}
