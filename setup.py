from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
  name='pyzenoh_bridge',
  version='0.1',
  rust_extensions=[
    RustExtension(
      'pyzenoh_bridge',
      'Cargo.toml',
      binding=Binding.PyO3
    )
  ],
  zip_safe=False
)