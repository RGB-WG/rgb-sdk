# RGB demo application

This React Native project has the purpose to show the capabilities of
the [RGB Node & SDK](https://github.com/LNP-BP/rgb-node) and to provide
a concrete example of integration of the library in a mobile application.

For detailed and platform-dependent setup instructions see the
[official React Native documentation](https://reactnative.dev/docs/environment-setup).

### Common requirements

- [Node](https://nodejs.org) 8.3 or newer

### Common build

From the project root, run:
```bash
$ npm install
```
This will install the React Native command line interface and other necessary
Node dependencies.

Start [Metro](https://github.com/facebook/metro), a JavaScript bundler for React Native, by running:
```bash
$ npx react-native start --port 8081
```

## Android

### Requirements

- Java SE Development Kit (JDK) 8
- Android development environment (Android Studio, SDK, AVD)

### Build and run

Build the rgb-node android bindings by following
[this instructions](/ffi/android)
and then copy the generated archive inside this project:

```bash
$ mkdir -p android/app/libs
$ cp <path_to_rgb-node_project>/ffi/android/library/build/outputs/aar/library-debug.aar android/app/libs/library-debug.aar
```

Open the android project from Android Studio
(`File -> Open... -> <path_to_this_project>/demo/react-native/android`),
build it (`Build -> Make project`) and
run the app (`Run -> Run 'app'` or `npx react-native run-android`).

## iOS

### Requirements

- Mac
- [Watchman](https://github.com/facebook/watchman)
- Xcode
