from click import BadParameter


def check_empty_string(ctx, param, value: str):
    if value == "":
        raise BadParameter("This parameter/option cannot be empty.")

    return value
