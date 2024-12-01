import collections

with open('i.txt') as f:
    lines = f.read().strip().split('\n')

a = 0

s = 0
ss = 0
gears = collections.defaultdict(list)
for i, line in enumerate(lines):
    l = line.strip()
    n = 0
    counts = False
    gear = False
    for j, c in enumerate(l):
        if c.isdigit():
            n = n*10 + int(c)
            for ii in [-1, 0, 1]:
                for jj in [-1, 0, 1]:
                    try:
                        cc = lines[i+ii][j+jj]
                        if cc != '.' and not cc.isdigit():
                            counts = True
                            if cc == '*':
                                gear = (i+ii,j+jj)
                    except IndexError:
                        continue
        if not c.isdigit() or j == len(l)-1:
            if counts:
                s += n
                a += 1; 
                print(a, n)
                counts = False
            if gear:
                gears[gear].append(n)
                if len(gears[gear]) == 2:
                    ss += n*gears[gear][0]
                gear = False
            n = 0

print(s)
print(ss)