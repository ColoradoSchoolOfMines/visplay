from setuptools import setup

setup(name='visplay',
      version='0.1',
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
          'requests'
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
)
