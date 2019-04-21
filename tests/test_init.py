from appdirs import AppDirs
from tgcli import appdirs


def test_isinstance_appdirs():
    assert isinstance(appdirs, AppDirs)
