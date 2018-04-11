from setuptools import setup
from sphinx.setup_command import BuildDoc

# Allow building sphinx documentation.
cmdclass = {'build_sphinx': BuildDoc}

name = 'visplay'
version = '0.1'
release = '0.1.0'

setup(
    name=name,
    version=release,
    description='Visplay',
    url='https://github.com/ColoradoSchoolOfMines/visplay',
    author='Mines ACM',
    author_email='Not yet',
    license='GPL3',
    packages=['visplay'],
    install_requires=[
        'python-mpv',
        'configparser',
        'pyyaml',
        'requests',
        'uri',
        'prompt',
    ],
    zip_safe=False,

    # To provide executable scripts, use entry points in preference to the
    # "scripts" keyword. Entry points provide cross-platform support and
    # allow pip to create the appropriate form of executable for the target
    # platform.
    entry_points={
        'console_scripts': [
            'visplay=visplay.__main__:main',
        ],
    },

    # Options for generating documentation.
    cmdclass=cmdclass,
    command_options={
        'build_sphinx': {
            'project': ('setup.py', name),
            'version': ('setup.py', version),
            'release': ('setup.py', release),
            'build_dir': ('setup.py', './build'),
        },
    },
)
