from vpn_config_parser import *

config_text: str

with open('./vpn_server.config', 'r', encoding='utf-8-sig') as content_file:
    config_text = content_file.read()

parse_result = parse_config(config_text)

def is_user_block(declare: Declare) -> bool:
    return declare.name == "UserList"

def flatten_iterator(declare: Declare):
    yield declare

    for declaration in declare.declarations:
        yield from flatten_iterator(declaration)

def get_users_blocks(declare: Declare):
    iterator = filter(is_user_block, flatten_iterator(declare))

    return iterator

def get_users(declare: Declare):
    users_blocks_iterator = get_users_blocks(declare)

    for user_block in users_blocks_iterator:
        if user_block is not None:
            yield from flatten_iterator(user_block)

if parse_result is not None:
    for user in get_users(parse_result):
        print(user.name)