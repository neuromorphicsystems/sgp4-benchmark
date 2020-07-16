project('propagate', 'cpp')
files = [
    'src/propagate.cpp',
    'third_party/json.hpp',
    'third_party/sgp4ext.h',
    'third_party/sgp4ext.cpp',
    'third_party/sgp4unit.h',
    'third_party/sgp4unit.cpp',
]
executable(
    'propagate',
    files,
    cpp_args: ['-std=c++17', '-DSGP4_GEOPOTENTIAL=wgs84', '-DSGP4_MODE=105']
)
executable(
    'propagate_afspc',
    files,
    cpp_args: ['-std=c++17', '-DSGP4_GEOPOTENTIAL=wgs72', '-DSGP4_MODE=97']
)
executable(
    'propagate_fastmath',
    files,
    cpp_args: ['-std=c++17', '-DSGP4_GEOPOTENTIAL=wgs84', '-DSGP4_MODE=105', '-ffast-math']
)
executable(
    'propagate_afspc_fastmath',
    files,
    cpp_args: ['-std=c++17', '-DSGP4_GEOPOTENTIAL=wgs72', '-DSGP4_MODE=97', '-ffast-math']
)
