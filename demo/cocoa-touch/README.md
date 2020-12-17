# RGB iOS demo application

In order to run this demo, build the rgb bindings by following
[these instructions](/bindings/swift) and then, from the project root, run:

```bash
mkdir demo/swift/Libraries
cp librgb/target/universal/release/librgb.a demo/swift/Libraries/
```

Open the iOS project from Xcode
(`File -> Open... -> <path_to_this_project>/demo/ios`),
build it (`Product -> Build`) and
run the app (`Product -> Run`).
