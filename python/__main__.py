from vpn_config_parser import *
from declare_itertools import *

config_text: str

with open('./vpn_server.config', 'r', encoding='utf-8-sig') as content_file:
    config_text = content_file.read()

parse_result = parse_config(config_text)

if parse_result is not None:
    for user in get_users(parse_result):
        print(user)