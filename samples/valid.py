import argparse
import numpy as np

import keras
import keras.layers
from keras import backend as K

STR_CONST = "str const"
int_var = 34

def func(int_val, float_val):
    if int_val > float_val:
        new_val = int_val - float_val
        print("new val: " + str(new_val))
    else
        new_val = int_val + float_val
        print("new val: " + str(new_val))

if __name__ == "__main__":
    func(34, 10.0)