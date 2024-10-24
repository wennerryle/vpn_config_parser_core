from vpn_config_parser import Declare

def flatten_declare_iterator(declare: Declare):
    yield declare

    for declaration in declare.declarations:
        yield from flatten_declare_iterator(declaration)

def get_users_blocks(declare: Declare):
    return filter(
        lambda declare: declare.name == "UserList",
        flatten_declare_iterator(declare)
    )

def get_users(declare: Declare):
    for user_block in get_users_blocks(declare):
        yield from user_block.declarations