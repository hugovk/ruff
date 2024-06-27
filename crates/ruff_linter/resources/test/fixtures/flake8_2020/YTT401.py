import sys
from sys import version, version as v

print(sys.version)

print(sys.version[:4])
print(version[:4])
print(v[:4])

# the tool is timid and only flags certain numeric slices
i = 4
print(sys.version[:i])
