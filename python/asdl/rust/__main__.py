from pathlib import Path

import click

import asdl
from . import GeneratorMode, write_source, AUTOGEN_MESSAGE

@click.command()
@click.argument('input-filename')
@click.option('--rust-file', '-R', 'rust_filename', type=click.Path(), required=True)
@click.option('--dump-module', '-D', is_flag=True)
@click.option(
    '--mode', '-m', 'mode_names',
    help="The mode of operation, specifying what to generate (default: only AST)",
    type=click.Choice(tuple(mode.value for mode in GeneratorMode)),
    default=("ast",), multiple=True
)
def generate(input_filename, rust_filename, mode_names=('ast',), dump_module=False):
    input_filename = Path(input_filename)
    rust_filename = Path(rust_filename)
    modes = [GeneratorMode(name) for name in mode_names]
    auto_gen_msg = AUTOGEN_MESSAGE.format("/".join(Path(__file__).parts[-2:]))
    mod = asdl.parse(input_filename)
    if dump_module:
        print('Parsed Module:')
        try:
            from prettyprinter import register_pretty, \
                install_extras, \
                pprint as pretty_print
        except ImportError:
            print("WARN: Failed to import 'prettyprinter'", file=sys.stderr)
            pretty_print = print
        else:
            install_extras()
        pretty_print(mod)
    if not asdl.check(mod):
        sys.exit(1)

    with rust_filename.open("w") as rust_file:
        rust_file.write(auto_gen_msg)

        write_source(mod, rust_file, modes=modes)

    print(f"{rust_filename}, regenerated.")

if __name__ == "__main__":
    generate()
