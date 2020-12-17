{
  "targets": [
    {
      "target_name": "rgblib",
      "sources": [ "swig_wrap.cxx" ],
      "libraries": [
           '-L<(module_root_dir)/../../librgb/target/release',
           '-lrgb',
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
