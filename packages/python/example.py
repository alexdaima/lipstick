import math
import numpy as np
import matplotlib.pyplot as plt
from datetime import datetime

plt.style.use("ggplot")

multiline_string = """
hello
"""

# Numbers
y = 2
y = 2.2

# Lists
x = [1, 2, 3, 4, 5, 1, 2, 3, 5]

# Dictionaries
x = {"1": 1, "2": 2, "3": 3, "4": 4, "5": 5}

# Tuples
x = (x, y)

# Sets
x = {1, 2, 3}

# Booleans
is_cool = True

# Classes
class Test(Base):

    def __init__(self, name):
        self.name = name

    # Decoraters
    @classmethod
    def test(cls):
        pass

# Functions
def say_hello():
    print("hello")

