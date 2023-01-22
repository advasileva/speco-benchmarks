import timeit
from case1 import measureWith
from case2 import measureWithout

def main():
    print(f'With polymorphism: {timeit.Timer(measureWith).timeit(number=1)} seconds')
    print(f'Without polymorphism: {timeit.Timer(measureWithout).timeit(number=1)} seconds')


if __name__ == '__main__':
   main()