# a = "ACCGGGTTTT"
# a = "CTCAGGTCCG"
a = input()
cont = 1
maior  = 0
ant = a[0]
for x in a[1:]:
    if x == ant:
        cont+=1

    else:
        if cont > maior:
            maior = cont
        cont = 1
    ant = x
    

if cont>maior:
    maior =cont

print(maior)