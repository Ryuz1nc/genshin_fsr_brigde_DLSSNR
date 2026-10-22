# OpenNR v0.7.14

This compatibility package includes the locally built Simplified Chinese OpenNR
ReShade add-on, the AMD Neural Rendering runtime v0.6.0, the neural-network
weights, and the optional `AMD_Neural_Feed.fx` shader.

When ReShade is selected in the package installer, the add-on and shader are
installed under `ReShade/reshade-shaders`. The runtime DLL and weights are
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
