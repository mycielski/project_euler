import math


from tqdm import tqdm


def factorial(n: int) -> bool:
    if n < 3:
        raise ValueError("n must be greater than 2")

    sum_of_factorials = 0
    for digit in str(n):
        sum_of_factorials += math.factorial(int(digit))

    return sum_of_factorials == n


cumulative_sum = 0
for i in tqdm(range(3, 100000)):
    if factorial(i):
        cumulative_sum += i
        print(cumulative_sum)
