#!/usr/bin/env python

'''
Having trouble getting build commands for compile_commands.json out
of scons so I can try c2rust, so doing it based on build output.

Used this to generate:

```
scons -c && scons build > build.log
rg '^clang' build.log | scripts/compile_commands.py  > compile_commands.json
c2rust transpile compile_commands.json
```

This leaves the `.rs` files in `build/`.

NOTE - currently c2rust just inlines all header files *after* running the
C pre-processor, this means that logic such as `ifdefs` will not be preserved
and the `.rs` files will reflect whichever platform c2rust was run on.
'''

import sys
import json

commands = []

for line in sys.stdin:
    file = line.split(' ')[-1].strip()
    if file.endswith('.c'):
        command = {
            'directory': '/Users/rhelmer/src/libprio',
            'command': line.strip(),
            'file': file
        }

        commands.append(command)

print json.dumps(commands)
