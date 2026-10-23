# OpenNR v0.7.14

This compatibility package includes the locally built Simplified Chinese OpenNR
ReShade add-on, the AMD Neural Rendering runtime v0.6.0, the neural-network
weights, and the optional `AMD_Neural_Feed.fx` shader.

Extract the complete archive and run `GenshinFSRBridgeInstaller.exe`. Select
the game executable and the optional **ReShade + RenoDX + OpenNR** component.
If ReShade is marked as missing, click its **Download** button to fetch the
official full add-on runtime before installing.
The add-on and shader remain under `payload/ReShade/reshade-shaders` and
are loaded through the configured ReShade search paths. The runtime DLL and weights are
copied beside `YuanShen.exe` or `GenshinImpact.exe`. The installer verifies the
size and SHA-256 of every bundled file before copying; it upgrades the v0.4.3
runtime from the previous compatibility package and stops if an unrecognized
file with either runtime filename already exists in the game directory.

OpenNR requires an AMD RDNA2, RDNA3, or RDNA4 GPU and a ReShade 6.x build with
full add-on support. RDNA2 cards need AMD HIP SDK 7.2; RX 7000 and RX 9000
drivers include the HIP 7 runtime. NVIDIA and Intel GPUs are unsupported.
Direct3D 11 is recommended. After starting the game, open ReShade's **Add-ons**
tab, enable **OpenNR**, then select **中文** from its Language control if
desired. The add-on is disabled by default. Chinese text requires a ReShade
ImGui font that includes CJK glyphs; this package does not include a font.

The source project and its upstream usage notes are available at
<https://github.com/zmodelerlover/dlss5-neural-amd>. The AMD runtime and
weights are from <https://github.com/danielblnc/DLSS-NR-on-AMD>. This package
does not include or use NVIDIA's `nvngx_dlssnr.dll`.

## Danielblnc NeuralHub v0.8.0

Danielblnc's standalone NeuralHub v0.8.0 supports DLSS Super Resolution, Ray
Reconstruction, Frame Generation, and Neural Rendering. Download
`NeuralHub_v0.8.0_setup.exe` directly from the
[official v0.8.0 release](https://github.com/danielblnc/DLSS-NR-on-AMD/releases/tag/v0.8.0).
Its license prohibits repackaging or redistribution, so this compatibility
package does not contain the installer.

NeuralHub's Neural Rendering feature requires the user's own `nvngx_dlssnr.dll`.
The other features can additionally use `nvngx_dlss.dll`, `nvngx_dlssd.dll`,
and `nvngx_dlssg.dll`. The four copies checked for this package are version
`310.9.16.0`; import the needed files into NeuralHub after
installing it. These NVIDIA files are not included in the public release.

NeuralHub's v0.8.0 documentation targets DirectX 12 games. This package's
Genshin FSR Bridge path uses DirectX 11, so NeuralHub is a separate optional
tool and is not verified as a replacement for the ReShade OpenNR integration
described above.
