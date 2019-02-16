people_num, food_num = [int(n) for n in input().split()]
_, *fav_foods = [int(n) for n in input().split()]
fav_foods = set(fav_foods)

for p in range(1, people_num):
    _, *favs = [int(n) for n in input().split()]
    favs = set(favs)
    fav_foods = {f for f in fav_foods if f in favs}

print(len(fav_foods))
