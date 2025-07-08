# Roadmap

## Interaction
- [X] User can pan by dragging mouse.
- [] User can change zoom with slider.
- [X] User can change zoom with Ctrl + Scroll.
    - [] User can change zoom using Scroll instead of Ctrl + Scroll.
- [X] User can change game speed with slider.
- [] User can change game speed by pressing keyboard numbers 1 to 5.
- [X] User can pause / unpause with a single button that changes text depending on state.
- [] User can pause /unpause game by pressing Space.
- [] User should be able to reset game by clicking button.

## Appearance
- [X] Extra Panel for all User-input config.
- [] On game startup, window should be fitted to screen.
- [] On game startup, zoom should be adjusted so that the field fills the window.
- [] Settings Panel should be located in bottom left corner, with a 5% margin to window border.
- [] User can close Settings window.
- [] User can open Settings screen with Gear icon.

## Logic
- [] Egui should only draw cells that have changed state.
- [] Field should increase in size right & down if birth event occurs outside current vector bounds.
- [] Field should increase in size left & up if birth event occurs outside current vector bounds.

# Issues
1. On the left side of the screen, seems like cells are culled. Zooming in makes left-side cells appear, and right-side cells disappear. Top/bottom similar.