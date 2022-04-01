from setuptools import setup
from setuptools_rust import Binding, RustExtension

extras = {}
extras["testing"] = ["pytest"]
extras["docs"] = ["setuptools_rust", "sphinx-copybutton", "sphinx_book_theme"]

setup(
    name="ethers",
    version="0.1.0",
    description="Python library for interacting with the Ethereum Blockchain ",
    long_description=open("README.md", "r", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    keywords="ethereum etherscan blockchain",
    author="Yaser Martinez Palenzuela",
    author_email="yaser.martinez@gmail.com",
    url="https://github.com/elyase/ethers-py",
    license="MIT",
    rust_extensions=[RustExtension("ethers.ethers", binding=Binding.PyO3, debug=False)],
    extras_require=extras,
    classifiers=[
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
    ],
    package_dir={"": "py_src"},
    packages=[
        "ethers",
        "ethers.providers",
    ],
    package_data={
        "ethers": ["py.typed", "__init__.pyi"],
        "ethers.providers": ["py.typed", "__init__.pyi"],
    },
    zip_safe=False,
)
