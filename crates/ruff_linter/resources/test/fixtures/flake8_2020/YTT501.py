import sys
from sys import version, version as v

print(sys.version)

print(sys.version[:5])
print(version[:5])
print(v[:5])

# the tool is timid and only flags certain numeric slices
i = 5
print(sys.version[:i])
