//! OpenNR payload verification and runtime deployment for the GUI installer.
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

struct Asset<'a> {
    relative: &'a str,
    size: u64,
    hash: &'a str,
    install: bool,
    previous: Option<(u64, &'a str)>,
}

const ASSETS: &[Asset<'_>] = &[
    Asset {
        relative: "reshade-shaders/Addons/amd-nr.addon64",
        size: 732672,
        hash: "860302a29e370194c57118e11a51b0b8402302702be7a1fc70f6cdc72283e8b5",
        install: false,
        previous: None,
    },
    Asset {
        relative: "reshade-shaders/Shaders/AMD_Neural_Feed.fx",
        size: 18395,
        hash: "a11bb2fb702475adbadd6cebf9dd3961425780b497a3b909989610a954f23c40",
        install: false,
        previous: None,
    },
    Asset {
        relative: "NeuralRuntime/dlssnr_amd_pass1.dll",
        size: 56677888,
        hash: "430be589020685d03f0ad92194457bafe1a186bf658c8cfe2fb7a0bdb7592cc4",
        install: true,
        previous: Some((
            12749824,
            "f3d9f2e53b775e4870917572f1f87a28c73068a4dc97252d6fb52360ddf8597a",
        )),
    },
    Asset {
        relative: "NeuralRuntime/dlssnr_on_amd_weights.bin",
        size: 147689451,
        hash: "6bf8dc931ef3ccffe18c82de26ab374156e7f19539ffcf8eabaa25dca5cf15ab",
        install: true,
        previous: None,
    },
];

fn fingerprint(path: &Path) -> Result<(u64, String), String> {
    let mut file =
        File::open(path).map_err(|e| format!("读取 OpenNR 文件失败 {}：{e}", path.display()))?;
    let size = file.metadata().map_err(|e| e.to_string())?.len();
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok((size, format!("{:x}", hasher.finalize())))
}

fn matches(value: &(u64, String), size: u64, hash: &str) -> bool {
    value.0 == size && value.1.eq_ignore_ascii_case(hash)
}

pub fn install(reshade_dir: &Path, game_dir: &Path) -> Result<(), String> {
    deploy(reshade_dir, game_dir, ASSETS)
}

fn deploy(reshade_dir: &Path, game_dir: &Path, assets: &[Asset<'_>]) -> Result<(), String> {
    let mut pending = Vec::new();
    // Check every source and destination before changing any game files.
    for asset in assets {
        let source = reshade_dir.join(asset.relative);
        if !matches(&fingerprint(&source)?, asset.size, asset.hash) {
            return Err(format!("OpenNR 包文件校验失败：{}", source.display()));
        }
        if !asset.install {
            continue;
        }
        let target = game_dir.join(source.file_name().ok_or("OpenNR 文件名无效")?);
        if target.try_exists().map_err(|e| e.to_string())? {
            let existing = fingerprint(&target)?;
            if matches(&existing, asset.size, asset.hash) {
                continue;
            }
            if !asset
                .previous
                .is_some_and(|(size, hash)| matches(&existing, size, hash))
            {
                return Err(format!(
                    "游戏目录已有未知版本的 OpenNR 文件，已保留：{}",
                    target.display()
                ));
            }
        }
        pending.push((asset, source, target));
    }
    for (asset, source, target) in pending {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let temporary = game_dir.join(format!(".opennr-{}-{nonce}.tmp", std::process::id()));
        let result = (|| {
            File::options()
                .write(true)
                .create_new(true)
                .open(&temporary)
                .map_err(|e| e.to_string())?;
            fs::copy(&source, &temporary).map_err(|e| format!("复制 OpenNR 文件失败：{e}"))?;
            if !matches(&fingerprint(&temporary)?, asset.size, asset.hash) {
                return Err("OpenNR 临时文件校验失败".to_string());
            }
            // std::fs::rename replaces an existing file on Windows as well.
            fs::rename(&temporary, &target)
                .map_err(|e| format!("部署 OpenNR 文件失败 {}：{e}", target.display()))
        })();
        if temporary.is_file() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NEW: &[u8] = b"current runtime";
    const OLD: &[u8] = b"previous runtime";

    fn hash(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    fn fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
        let root = tempfile::tempdir().unwrap();
        let payload = root.path().join("ReShade");
        let game = root.path().join("game");
        fs::create_dir_all(payload.join("NeuralRuntime")).unwrap();
        fs::create_dir(&game).unwrap();
        fs::write(payload.join("NeuralRuntime/runtime.dll"), NEW).unwrap();
        (root, payload, game)
    }

    #[test]
    fn fresh_install_and_second_run_preserve_current_file() {
        let (_root, payload, game) = fixture();
        let digest = hash(NEW);
        let assets = [Asset {
            relative: "NeuralRuntime/runtime.dll",
            size: NEW.len() as u64,
            hash: &digest,
            install: true,
            previous: None,
        }];
        deploy(&payload, &game, &assets).unwrap();
        let modified = fs::metadata(game.join("runtime.dll"))
            .unwrap()
            .modified()
            .unwrap();
        deploy(&payload, &game, &assets).unwrap();
        assert_eq!(fs::read(game.join("runtime.dll")).unwrap(), NEW);
        assert_eq!(
            fs::metadata(game.join("runtime.dll"))
                .unwrap()
                .modified()
                .unwrap(),
            modified
        );
        assert_eq!(fs::read_dir(&game).unwrap().count(), 1);
    }

    #[test]
    fn recognized_old_runtime_is_replaced_on_windows() {
        let (_root, payload, game) = fixture();
        fs::write(game.join("runtime.dll"), OLD).unwrap();
        let digest = hash(NEW);
        let previous = hash(OLD);
        deploy(
            &payload,
            &game,
            &[Asset {
                relative: "NeuralRuntime/runtime.dll",
                size: NEW.len() as u64,
                hash: &digest,
                install: true,
                previous: Some((OLD.len() as u64, &previous)),
            }],
        )
        .unwrap();
        assert_eq!(fs::read(game.join("runtime.dll")).unwrap(), NEW);
    }

    #[test]
    fn unknown_target_is_preserved_and_no_other_file_is_installed() {
        let (_root, payload, game) = fixture();
        fs::write(game.join("runtime.dll"), b"user file").unwrap();
        fs::write(payload.join("NeuralRuntime/weights.bin"), b"weights").unwrap();
        let digest = hash(NEW);
        let weights = hash(b"weights");
        let assets = [
            Asset {
                relative: "NeuralRuntime/weights.bin",
                size: 7,
                hash: &weights,
                install: true,
                previous: None,
            },
            Asset {
                relative: "NeuralRuntime/runtime.dll",
                size: NEW.len() as u64,
                hash: &digest,
                install: true,
                previous: None,
            },
        ];
        assert!(deploy(&payload, &game, &assets)
            .unwrap_err()
            .contains("未知版本"));
        assert_eq!(fs::read(game.join("runtime.dll")).unwrap(), b"user file");
        assert!(!game.join("weights.bin").exists());
    }

    #[test]
    fn corrupt_addon_blocks_runtime_copy() {
        let (_root, payload, game) = fixture();
        fs::write(payload.join("addon"), b"corrupt").unwrap();
        let digest = hash(NEW);
        let expected = hash(b"correct");
        let assets = [
            Asset {
                relative: "NeuralRuntime/runtime.dll",
                size: NEW.len() as u64,
                hash: &digest,
                install: true,
                previous: None,
            },
            Asset {
                relative: "addon",
                size: 7,
                hash: &expected,
                install: false,
                previous: None,
            },
        ];
        assert!(deploy(&payload, &game, &assets)
            .unwrap_err()
            .contains("校验失败"));
        assert_eq!(fs::read_dir(&game).unwrap().count(), 0);
    }

    #[test]
    fn missing_payload_blocks_deployment() {
        let (_root, payload, game) = fixture();
        assert!(install(&payload, &game).is_err());
        assert_eq!(fs::read_dir(&game).unwrap().count(), 0);
    }

    #[test]
    fn real_release_payload_deploys_and_is_idempotent() {
        let Some(payload) = std::env::var_os("GFSR_OPENNR_TEST_PAYLOAD") else {
            return;
        };
        let game = tempfile::tempdir().unwrap();
        install(Path::new(&payload), game.path()).unwrap();
        install(Path::new(&payload), game.path()).unwrap();
        for asset in ASSETS.iter().filter(|a| a.install) {
            let name = Path::new(asset.relative).file_name().unwrap();
            assert!(matches(
                &fingerprint(&game.path().join(name)).unwrap(),
                asset.size,
                asset.hash
            ));
        }
        assert_eq!(fs::read_dir(game.path()).unwrap().count(), 2);
    }
}
