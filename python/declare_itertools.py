import numpy as np
import pandas as pd
from vpn_config_parser import Declare

def flatten_declare_iterator(declare: Declare):
    yield declare

    for declaration in declare.declarations:
        yield from flatten_declare_iterator(declaration)

def get_users_blocks(declare: Declare):
    iterator = filter(
        lambda declare: declare.name == "UserList",
        flatten_declare_iterator(declare)
    )
    return iterator

def get_users(declare: Declare):
    users_blocks_iterator = get_users_blocks(declare)

    for user_block in users_blocks_iterator:
        if user_block is not None:
            yield from user_block.declarations