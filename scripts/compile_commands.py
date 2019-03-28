#!/usr/bin/env python

'''
Having trouble getting build commands for compile_commands.json out
of scons so I can try c2rust, so doing it based on build output.
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
