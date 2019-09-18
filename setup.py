from setuptools import setup

import tgcli

with open("README.md", "r") as f:
    README = f.read()

with open("requirements.txt", "r") as f:
    DEPS = f.readlines()

with open("dev.requirements.txt", "r") as f:
    TEST_DEPS = f.readlines()

GITHUB_RELEASE_URL = "https://github.com/erayerdin/tgcli/archive/v{}.tar.gz"

setup(
    name="tgcli",
    version=tgcli.__version__,
    description="tgcli is a client tool for Telegram.",
    long_description=README,
    long_description_content_type="text/markdown",
    url="https://github.com/erayerdin/tgcli",
    download_url=GITHUB_RELEASE_URL.format(tgcli.__version__),
    packages=("tgcli", "tgcli.request", "tgcli.cli", "tgcli.cli.bot"),
    entry_points="""
        [console_scripts]
        tgcli=tgcli.cli:cli
    """,
    include_package_data=True,
    keywords="telegram messaging communication terminal tool cli",
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Intended Audience :: System Administrators",
        "License :: OSI Approved :: Apache Software License",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Topic :: Communications :: Chat",
    ],
    author=tgcli.__author__,
    author_email="eraygezer.94@gmail.com",
    license="Apache License 2.0",
    tests_require=TEST_DEPS,
    install_requires=DEPS,
    zip_safe=False,
)
