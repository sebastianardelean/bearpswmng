## Commands

Build under windows
```
cmake --build . --config Debug


cmake .. -DCMAKE_BUILD_TYPE=Debug
cmake --build . --config Debug


cmake -B build .. -G "Visual Studio 17 2022"
cmake --build build --clean-first --verbose
```


