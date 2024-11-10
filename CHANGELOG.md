# CHANGELOG - pixel_loop

# 0.2.0 - 10.11.2024

- Fix: Enable `InMemoryCanvas` regardless of `image-load` feature flag

- Change: Restructure feature flag naming: `image-load` -> `stb-image`

- Change: Moved winit intialization into the `PixelsCanvas`

- Feature: Glued InputStates to corresponding RenderableCanvas implementation via trait internal type.

- Change: Refactor Resizing handling and introduce `did_resize` state function.

- Change: Updated examples to represent changed API


# 0.1.1 - 06.11.2024

- Fix of flipped green and blue color components in `Color` factories.

# 0.1.0 - 04.11.2024

- First release as a standalone library on cargo.io
