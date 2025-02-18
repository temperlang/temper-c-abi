Some example steps:

```
PS C:\Users\work2\Documents\projects\temper-c-abi\examples\hello\target\debug> cargo build
... lots of warnings ...
PS C:\Users\work2\Documents\projects\temper-c-abi\examples\hello\target\debug> python ..\..\examples\greet.py
Hello, world!
PS C:\Users\work2\Documents\projects\temper-c-abi\examples\hello\target\debug> cl ..\..\examples\greet.c hello.dll.lib /link
Microsoft (R) C/C++ Optimizing Compiler Version 19.42.34438 for x64
Copyright (C) Microsoft Corporation.  All rights reserved.

greet.c
Microsoft (R) Incremental Linker Version 14.42.34438.0
Copyright (C) Microsoft Corporation.  All rights reserved.

/out:greet.exe
greet.obj
hello.dll.lib
PS C:\Users\work2\Documents\projects\temper-c-abi\examples\hello\target\debug> .\greet.exe
Hello, world!
PS C:\Users\work2\Documents\projects\temper-c-abi\examples\hello\target\debug>
```
