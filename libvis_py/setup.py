from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='libvis',
    version='0.0.0',
    url='https://github.com/ColoradoSchoolOfMines/visplay',
    author='Mines ACM',
    license='GPL3',

    rust_extensions=[RustExtension(
        'libvis._vis',
        'Cargo.toml',
        binding=Binding.PyO3)
    ],
    packages=['libvis'],
    # specify setup dependencies
    setup_requires=[
        'setuptools',
        'setuptools_rust',
    ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False
)
