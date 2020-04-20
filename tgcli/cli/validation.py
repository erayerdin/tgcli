from click import BadParameter


def check_empty_string(ctx, param, value: str):
    if value == "":
        raise BadParameter("This parameter/option cannot be empty.")

    return value


def check_positive_integer(ctx, param, value: int):
    if value < 0:
        raise BadParameter("This parameter/option cannot be a negative integer.")

    return value
