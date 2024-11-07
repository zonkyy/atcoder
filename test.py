n = int(input())
a_s = [0] + [int(i) for i in input().split()]
b_s = [0, 0] + [int(i) for i in input().split()]

ans = [0, a_s[1]]
routes = [0, 0]

for i in range(2, n):
    if ans[i - 1] + a_s[i] < ans[i - 2] + b_s[i]:
        route = i - 1
        a = ans[i - 1] + a_s[i]
    else:
        route = i - 2
        a = ans[i - 2] + b_s[i]
    routes.append(route)
    ans.append(a)

r = routes[-1]
ans = [n]
while r != 0:
    ans.append(r + 1)
    r = routes[r]
ans.append(r + 1)

print(ans)

# print(len(ans))
# [print(a, end=" ") for a in ans[::-1]]
