import timeit
start = timeit.default_timer()

i = 0
f = 5200.0
while i < 10000000:
    f = 0.62*f + 1900.0
    i+=1
print(f)

stop = timeit.default_timer()
print('Time: ', stop - start) 