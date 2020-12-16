# RGB iOS demo application

In order to run this demo, build the rgb bindings by following
[these instructions](/ffi/ios) and then, from the project root, run:

```bash
mkdir demo/ios/Libraries
cp librgb/target/universal/release/librgb.a demo/ios/Libraries/
cp <path_to_libzmq>/builds/ios/lib/libzmq.a demo/ios/Libraries/
```

Open the iOS project from Xcode
(`File -> Open... -> <path_to_this_project>/demo/ios`),
build it (`Product -> Build`) and
run the app (`Product -> Run`).
