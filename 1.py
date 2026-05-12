with open('lab4_input.txt', 'r') as f:
    lines = f.readlines()
    for q in lines:
        a, b = map(float, q.split())
        if a > 100 or b > 100:
            print(a, b)