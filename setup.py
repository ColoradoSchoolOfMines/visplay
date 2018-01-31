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
      ],
      zip_safe=False)
