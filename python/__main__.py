from vpn_config_parser import *

config_text: str

with open('./vpn_server.config', 'r', encoding='utf-8-sig') as content_file:
    config_text = content_file.read()
    print(config_text)

parse_result = parse_config(config_text)

print(parse_result)

if parse_result is not None:
    for constant in parse_result.constants:
        print(f'type {constant.type} | key {constant.key} | value {constant.value}')