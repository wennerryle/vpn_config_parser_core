# PS: __init__ for Declare and Constant are not created in Rust. It's required just for typing

from vpn_config_parser.vpn_config_parser import *

config_text: str

with open('./vpn_server.config') as content_file:
    config_text = content_file.read()

parse_config()