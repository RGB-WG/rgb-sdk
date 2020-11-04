# RGB Android demo application

### Build and run

In order to run this demo, build the rgb bindings by following
[these instructions](/ffi/android) and then, from the project root, run:

```bash
$ mkdir demo/android/app/libs
$ cp ffi/android/library/build/outputs/aar/library-debug.aar demo/android/app/libs/
```

Open the android project from Android Studio
(`File -> Open... -> <path_to_this_project>/demo/android`),
build it (`Build -> Make project`) and
run the app (`Run -> Run 'app'`).
