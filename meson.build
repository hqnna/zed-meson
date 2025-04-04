project(
    'random', 'cpp',
    version: '1.0.0',
    license: 'mpl-2.0',
    default_options: [
        'warning_level=3',
        'cpp_std=c++17',
        'werror=true'
    ]
)

includes = include_directories('include')
src = run_command('scripts/glob.sh').stdout().strip().split('\n')
executable(meson.project_name(), src, include_directories: includes)
