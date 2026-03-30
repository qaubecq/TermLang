import time

start_time = time.time()

n = 0
while True:
    n += 1
    if n == 4294967296 : break

print("Without u8 : " + str(time.time()-start_time))



start_time = time.time()

try:
    mem = [0, 0, 0, 0]
    while True:
        index = 0
        while True:
            old = mem[index]
            mem[index] = mem[index] + 1
            if old == 255:
                mem[index] = 0
                index += 1
            else:
                break
except:
    print("With u8 : " + str(time.time()-start_time))