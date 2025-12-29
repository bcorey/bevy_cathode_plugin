## Flatscreen CRT Plugin for Bevy
This plugin applies a customizable CRT postprocessing stage to both the 3D view and the GUI on top of that 3d view. You will need to click on the below image for it to render in a way that looks appealing.

<img width="1423" height="830" alt="Screenshot 2025-09-27 at 6 28 13 PM" src="https://github.com/user-attachments/assets/d96bf3d1-3555-4beb-847a-d25da65db6ee" />

## WASM Requirements
WebGPU must be enabled for WASM support.

### Attributions
The 'Complex CRT' shader is sourced from https://github.com/cmhhelgeson/WGSL_Shader_Depot
The postprocessing pipeline is derived from the bevy examples and altered to apply to the UI layer in addition to the 3d world.
