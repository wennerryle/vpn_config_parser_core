from typing import Optional
from vpn_config_parser import *
from declare_itertools import *
import converters
import pandas as pd

config_text: str

with open('./vpn_server.config', 'r', encoding='utf-8-sig') as content_file:
    config_text = content_file.read()

parse_result = parse_config(config_text)

def process_config(parse_result: Declare) -> pd.DataFrame:
    users = [*get_users(parse_result)]
    return converters.flatten_declare_objects(users)

if parse_result is not None:
    users_df = process_config(parse_result)
    print(users_df)

    users_df.to_excel("text.xlsx")