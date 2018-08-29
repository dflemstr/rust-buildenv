#!/usr/bin/env python2
import sys

if sys.argv[1] == 'dist':
    for i in xrange(2, len(sys.argv) - 1):
        if sys.argv[i] == '--target':
            for target_fragment in sys.argv[i + 1:]:
                for target in target_fragment.split(','):
                    print target,
