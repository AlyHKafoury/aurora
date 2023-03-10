import time

def fibonacci(n):
    if n < 2:
            return n
    return fibonacci(n-1) + fibonacci(n-2)

start = time.time()
print(fibonacci(30))
end = time.time()
print(f"Time taken: {end - start} seconds")