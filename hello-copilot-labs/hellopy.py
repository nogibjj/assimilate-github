"""A really cool hello world python script


"""
import colorama
from colorama import Fore, Back, Style
import time

colorama.init()

colors = [Fore.RED, Fore.GREEN, Fore.YELLOW, Fore.BLUE, Fore.MAGENTA, Fore.CYAN, Fore.WHITE]

for letter in "Hello, World!":
    print(letter, end='', flush=True)
    for color in colors:
        print(color, end='', flush=True)
        print(Back.RESET + letter, end='', flush=True)
        time.sleep(0.1)

print(Style.RESET_ALL)
