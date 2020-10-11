{
  "targets": [
    {
      "target_name": "rgb_node",
      "sources": [ "swig_wrap.cxx" ],
      "libraries": [
           '-L<(module_root_dir)/../../rust-lib/target/debug/',
           '-lrgb',
       ],
      'include_dirs': [
           '../../rust-lib',
       ],
       "ldflags": [
           '-Wl,-rpath,../../rust-lib/target/debug/'
       ],
      "cflags!": ["-std=c++11"],
    }
  ]
}
