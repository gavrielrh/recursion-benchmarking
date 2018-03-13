import sys

// Increase recursive call stack size
sys.setrecursionlimit(1500)

// Recursive Fibonacci
def fibo_rec(n):
    if n <= 2:
        return 1

    return fibo_rec(n - 1) + fibo_rec(n - 2)

// Iterative Fibonacci
def fibo_iter(n):
    a = 1
    b = 1
    for _ in range(3, n+1):
        c = a + b
        a = b
        b = c

    return b

// Helper method for Quicksort
def partition(a, low, high):
    i = low - 1
    pivot = a[high]

    for j in range(low, high):
        if a[j] <= pivot:
            i = i + 1
            a[i], a[j] = a[j], a[i]

    a[i + 1], a[high] = a[high], a[i + 1]

    return i + 1

// Wrapper for recursive quicksort
def quicksort_rec(a):
    real_quicksort_rec(a, 0, len(a) - 1)

// Recursive quicksort
def real_quicksort_rec(a, low, high):
    if low < high:
        p = partition(a, low, high)

        real_quicksort_rec(a, low, p - 1)
        real_quicksort_rec(a, p+1, high)

// Iterative quicksort using a stack
def quicksort_iter(a):
    stack = []
    start = 0
    end = len(a) - 1

    stack.append((start, end))

    while len(stack) > 0:
        start, end = stack.pop()

        pivot = partition(a, start, end)

        if pivot - 1 > start:
            stack.append((start, pivot - 1))

        if pivot + 1 < end:
            stack.append((pivot + 1, end))
    
// Reads a list from file at filepath
def array_from_file(filepath):
    text_file = open(filepath, "r")
    line = text_file.read().split(', ')[1:-1]
    array = [int(i) for i in line]
    text_file.close()
    return array

def main():
    x = array_from_file("vec10.txt")
    print(x)
    #quicksort_rec(x, 0, len(x) - 1)
    quicksort_iter(x)
    print(x)

main()

// Test using 'pytest main.py'
def test_fibo_rec(benchmark):
    result = benchmark(fibo_rec, 50)

def test_fibo_iter(benchmark):
    result = benchmark(fibo_iter, 50)

def test_quicksort_rec(benchmark):
    a = array_from_file("vec1000.txt")
    result = benchmark(quicksort_rec, a)

def test_quicksort_iter(benchmark):
    a = array_from_file("vec1000.txt")
    result = benchmark(quicksort_iter, a)