# PS: __init__ for Declare and Constant are not created in Rust. It's required just for typing

from vpn_config_parser import parse_config, Declare, Constant

config_text: str

with open('./vpn_server.config', encoding="utf-8") as content_file:
    config_text = content_file.read()

parse_result = parse_config(config_text)

if parse_result != None:
    print("parsed!", parse_result)