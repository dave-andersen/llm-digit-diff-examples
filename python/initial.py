import random
import time

# Generate a list of 1 million random integers between 1 and 100,000
random_integers = [random.randint(1, 100000) for _ in range(1000000)]

def digit_sum(n):
    return sum(int(digit) for digit in str(n))

# Filter numbers whose digits sum up to 30
start = time.time()
filtered_numbers = [num for num in random_integers if digit_sum(num) == 30]

if filtered_numbers:
    min_number = min(filtered_numbers)
    max_number = max(filtered_numbers)
    difference = max_number - min_number
    end = time.time()
    print(f"Time taken to find the numbers: {end - start} seconds")
    print(f"The difference between the largest and smallest numbers whose digits sum up to 30 is: {difference}")
else:
    print("No numbers found with digits summing up to 30.")