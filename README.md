# rutilities
chilean rut utilities in rust and wasm

## Install

**Node and browsers:**
```bash
npm install rutilities
```

**Rust:**
```toml
[dependencies]
rutilities = "0.2"
```

## API

clean_rut / cleanRut: `"14.780.556-k" -> "14780556K"`

format_rut / formatRut: `"14780556k" -> "14.780.556-K"`

is_rut_valid / isRutValid: `"14.780.556-K" -> true`

generate_valid_rut / generateValidRut: `_ -> "14780556K"`

