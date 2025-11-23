# Run pytest
[working-directory: 'crates/textum-py']
pyt *args:
   $(uv python find) -m pytest {{args}}

# Run maturin
[working-directory: 'crates/textum-py']
mat *args:
   maturin develop --uv {{args}}
