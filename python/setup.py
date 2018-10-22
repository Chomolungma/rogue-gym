""" setup.py for rogue_gym """
import os
import subprocess
import sys
from setuptools import setup
from setuptools import find_packages
from setuptools.command.test import test as TestCommand


try:
    from setuptools_rust import RustExtension
except ImportError:
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension


class CmdTest(TestCommand):
    def run(self):
        self.run_command("test_rust")
        test_files = os.listdir('./tests')
        for f in test_files:
            _, ext = os.path.splitext(f)
            if ext == '.py':
                subprocess.check_call([sys.executable, f], cwd='./tests')


setup_requires = ['setuptools-rust>=0.6.0']
install_requires = ['numpy', 'gym']
test_requires = install_requires

setup(
    name='rouge-gym',
    version='0.1.0',
    description='OpenAI gym environment for rogue-gym',
    url='https://github.com/kngwyu/rogue-gym',
    author='Yuji Kanagawa',
    author_email='yuji.kngw.80s.revive@gmail.com',
    classifiers=[
        'License :: OSI Approved :: MIT License',
        'License :: OSI Approved :: Apache Software License',
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'Programming Language :: Python',
        'Programming Language :: Rust',
        'Operating System :: POSIX',
    ],
    packages=find_packages(),
    rust_extensions=[RustExtension('rogue_gym_python._rogue_gym', 'Cargo.toml')],
    install_requires=install_requires,
    test_requires=test_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict(test=CmdTest)
)
