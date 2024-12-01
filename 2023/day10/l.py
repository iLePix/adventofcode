from collections import defaultdict, deque

N, S, E, W = -1, +1, +1j, -1j

exits = {'|': (N, S), '-': (E, W),
         'L': (N, E), 'J': (N, W),
         '7': (S, W), 'F': (S, E),
         '.': (), 'S': (N, E, S, W)}

board = {(p:=2*i+2j*j): [p+e for e in exits[c]]
    for i,r in enumerate(open('i.txt'))
    for j,c in enumerate(r.strip())}

start = next(k for k,v in board.items() if len(v)==4)
graph = defaultdict(set)

for p, qs in board.items():
    for q in qs:
        graph[p].add(q)
        graph[q].add(p)

dist = defaultdict(lambda: 1_000_000)

q = deque([(start,0)])
while q:
    n, d = q.popleft()
    dist[n] = d
    for e in graph[n]:
        if dist[e] > d: q.append((e, d+1))

print(max(dist.values())//2)