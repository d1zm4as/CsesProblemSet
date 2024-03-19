

n = int(input())

lista =list(map(int, input().split()))  

print(sum(list(range(n+1)))-sum(lista))