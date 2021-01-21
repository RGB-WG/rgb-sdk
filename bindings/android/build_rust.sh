#!/usr/bin/env bash
set -eo pipefail

RUSTLIB="../../librgb"

cargo build --manifest-path $RUSTLIB/Cargo.toml

# Update this line accordingly if you are not building *from* x86_64
export PATH=$PATH:$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin

export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android26-clang"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang"
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang"

CC="aarch64-linux-android21-clang" CFLAGS="--sysroot=$NDK_HOME/sysroot -I$NDK_HOME/sysroot/usr/include -I$NDK_HOME/sysroot/usr/include/aarch64-linux-android" CXX="aarch64-linux-android21-clang++" CXXFLAGS="$CFLAGS -I$NDK_HOME/sources/cxx-stl/llvm-libc++/include" LDFLAGS="--sysroot=$NDK_HOME/platforms/android-21/arch-arm64" cargo build --manifest-path $RUSTLIB/Cargo.toml --target=aarch64-linux-android
CC="x86_64-linux-android21-clang" CFLAGS="--sysroot=$NDK_HOME/sysroot -I$NDK_HOME/sysroot/usr/include -I$NDK_HOME/sysroot/usr/include/x86_64-linux-android" CXX="x86_64-linux-android21-clang++" CXXFLAGS="$CFLAGS -I$NDK_HOME/sources/cxx-stl/llvm-libc++/include" LDFLAGS="--sysroot=$NDK_HOME/platforms/android-21/arch-x86_64" cargo build --manifest-path $RUSTLIB/Cargo.toml --target=x86_64-linux-android
CC="armv7a-linux-androideabi21-clang" CFLAGS="--sysroot=$NDK_HOME/sysroot -I$NDK_HOME/sysroot/usr/include -I$NDK_HOME/sysroot/usr/include/arm-linux-androideabi" CXX="armv7a-linux-androideabi21-clang++" CXXFLAGS="$CFLAGS -I$NDK_HOME/sources/cxx-stl/llvm-libc++/include" LDFLAGS="--sysroot=$NDK_HOME/platforms/android-21/arch-arm -L$NDK_HOME/sources/cxx-stl/llvm-libc++/libs/armeabi-v7a" cargo build --manifest-path $RUSTLIB/Cargo.toml --target=armv7-linux-androideabi
CC="i686-linux-android21-clang" CFLAGS="--sysroot=$NDK_HOME/sysroot -I$NDK_HOME/sysroot/usr/include -I$NDK_HOME/sysroot/usr/include/i686-linux-android" CXX="i686-linux-android21-clang++" CXXFLAGS="$CFLAGS -I$NDK_HOME/sources/cxx-stl/llvm-libc++/include" LDFLAGS="--sysroot=$NDK_HOME/platforms/android-21/arch-x86" cargo build --manifest-path $RUSTLIB/Cargo.toml --target=i686-linux-android

JNILIBS="library/src/main/jniLibs"

mkdir -pv library/src/main/java/org/lnpbp/rgb_autogen
swig -java -c++ -package "org.lnpbp.rgb_autogen" -outdir library/src/main/java/org/lnpbp/rgb_autogen swig.i

mkdir -p $JNILIBS/arm64-v8a $JNILIBS/x86_64 $JNILIBS/armeabi-v7a $JNILIBS/x86

aarch64-linux-android21-clang++ -static-libstdc++ swig_wrap.cxx -L$RUSTLIB/target/aarch64-linux-android/debug/ -lrgb_node -o $JNILIBS/arm64-v8a/librgb_node.so -fPIC
cp -v $RUSTLIB/target/aarch64-linux-android/debug/librgb_node.so $JNILIBS/arm64-v8a/
cp -v $NDK_HOME/sources/cxx-stl/llvm-libc++/libs/arm64-v8a/libc++_shared.so $JNILIBS/arm64-v8a/

x86_64-linux-android21-clang++ -static-libstdc++ swig_wrap.cxx -L$RUSTLIB/target/x86_64-linux-android/debug/ -lrgb_node -o $JNILIBS/x86_64/librgb_node.so -fPIC
cp -v $RUSTLIB/target/x86_64-linux-android/debug/librgb_node.so $JNILIBS/x86_64/
cp -v $NDK_HOME/sources/cxx-stl/llvm-libc++/libs/x86_64/libc++_shared.so $JNILIBS/x86_64/

armv7a-linux-androideabi21-clang++ -static-libstdc++ swig_wrap.cxx -L$RUSTLIB/target/armv7-linux-androideabi/debug/ -lrgb_node -o $JNILIBS/armeabi-v7a/librgb_node.so -fPIC
cp -v $RUSTLIB/target/armv7-linux-androideabi/debug/librgb_node.so $JNILIBS/armeabi-v7a/
cp -v $NDK_HOME/sources/cxx-stl/llvm-libc++/libs/armeabi-v7a/libc++_shared.so $JNILIBS/armeabi-v7a/

i686-linux-android21-clang++ -static-libstdc++ swig_wrap.cxx -L$RUSTLIB/target/i686-linux-android/debug/ -lrgb_node -o $JNILIBS/x86/librgb_node.so -fPIC
cp -v $RUSTLIB/target/i686-linux-android/debug/librgb_node.so $JNILIBS/x86/
cp -v $NDK_HOME/sources/cxx-stl/llvm-libc++/libs/x86/libc++_shared.so $JNILIBS/x86/
