import os

import pytest
import requests

try:
    requests.get("https://telegram.org")
    _HAS_CONNECTED = True
except Exception:
    _HAS_CONNECTED = False


_BOT_TOKEN_EXISTS = True
if os.environ.get("TELEGRAM_BOT_TOKEN") is None:
    _BOT_TOKEN_EXISTS = False


_BOT_RECEIVER_EXISTS = True
if os.environ.get("TELEGRAM_RECEIVER") is None:
    _BOT_RECEIVER_EXISTS = False


SKIPIF_HAS_NOT_CONNECTED = pytest.mark.skipif(
    not _HAS_CONNECTED,
    reason="This test requires a stable connection to telegram.org.",
)
SKIPIF_BOT_TOKEN_NOT_EXISTS = pytest.mark.skipif(
    not _BOT_TOKEN_EXISTS,
    reason="This test requires TELEGRAM_BOT_TOKEN environment variable to be set.",
)
SKIPIF_BOT_RECEIVER_NOT_EXISTS = pytest.mark.skipif(
    not _BOT_RECEIVER_EXISTS,
    reason="This test requires TELEGRAM_RECEIVER environment variable to be set.",
)
