# Sled issue reproduction

This reproduces an issue with auto flushing on android. To build the rust project you will need https://github.com/bbqsrc/cargo-ndk.

Then change to `rust` directory and run:

```sh
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o ../app/src/main/jniLibs build --release
```

Once these are built please open the `MainActivity.kt` file in Android studio and follow instructions on commenting out 
and commenting in the pieces of code. Please open logcat and filter to the word "RUST".

These lines, first of all update the db without flushing and then query the db after a restart of the app.
This should show a result of "none" as the update wasn't persisted.
After this it updates with a flush and querying then should result in a result of "VAL1".
