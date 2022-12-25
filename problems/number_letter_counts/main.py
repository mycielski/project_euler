import sys


from num2words import num2words


def num_to_word(number: int) -> str:
    return num2words(number).replace(" ", "").replace("-", "")


if __name__ == "__main__":
    count = 0
    for i in range(int(sys.argv[1])):
        count += len(num_to_word(i + 1))
    print(count)
