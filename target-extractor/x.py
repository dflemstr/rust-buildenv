#!/usr/bin/env python2
import sys

if sys.argv[1] == 'dist':
    for i in xrange(2, len(sys.argv) - 1):
        if sys.argv[i] == '--target':
            for target in sys.argv[i + 1:]:
                print target,
