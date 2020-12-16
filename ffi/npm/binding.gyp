{
  "targets": [
    {
      "target_name": "rgb",
      "sources": [ "swig_wrap.cxx" ],
      "libraries": [
           '<(module_root_dir)/../../librgb/target/release/librgb.a',
       ],
      "include_dirs": [
           '../../librgb',
       ],
      "ldflags": [
           '-Wl,-rpath,../../librgb/target/release/'
       ],
      "cflags!": ["-std=c++11"],
    }
  ]
}
