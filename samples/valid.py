
#! imports foram ignorados no sintatico
# import argparse
# import numpy as np

# import keras
# import keras.layers
# from keras import backend as K

STR_CONST = "str \n\'\"const"
int_var = 34

def func(int_val, float_val):
    a = int_val > float_val
    
    while True:
        if int_val > float_val or True and 1 == 1:
            new_val = int_val - float_val
            print("new val: " + str(new_val))
        elif True:
            new_val = int_val + float_val
            print("new val: " + str(new_val))
        #! por razões até o momento desconhecidas, isso não funciona
        # else:
        #     new_val = int_val + float_val
        #     print("new val: " + str(new_val))

    return 1

    #! dicts foram ignorados no sintatico

    # d = {
    #     "key": value
    # }

if __name__ == "__main__":
    func(34, 10.0)