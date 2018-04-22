import sys


def main():
    if sys.argv[1] == 'dist':
        for i in xrange(2, len(sys.argv) - 1):
            if sys.argv[i] == '--target':
                print sys.argv[i + 1],


if __name__ == '__main__':
    main()
