# Changelog

## v0.1.2
- Fix a possible panic when setting subimage data

## v0.1.1
- FIX: Don't crash when creating non-pow-2 textures
- Add no_std support (though glow still requires std)
- Update the version of blinds in the dev-dependency

## v0.1.0
- [Breaking] Add mipmap texture filter variants, and error cases for when mipmaps are unavailable
- Non-power-of-2 textures are now supported, but they don't have mipmaps
- Add methods for the size of a surface

## v0.1.0-alpha6
- Indicate the default blend mode in the docs
- Rework the Surface API to prevent texture loops
