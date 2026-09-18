use std::path::Path;
use widya::mobile::{
    bangun_aplikasi_mobile, inisialisasi_proyek_mobile, AndroidArch, IOSArch,
    MobileCompiler, MobileConfig,
};

#[test]
fn test_mobile_config_defaults() {
    let config = MobileConfig::default();
    assert!(config.android_enabled);
    assert!(config.ios_enabled);
    assert_eq!(config.android.min_sdk, 21);
    assert_eq!(config.android.target_sdk, 34);
    assert_eq!(config.android.package_name, "com.widya.mobileapp");
    assert_eq!(config.ios.min_version, "14.0");
    assert_eq!(config.ios.bundle_id, "com.widya.mobileapp");
}

#[test]
fn test_mobile_architectures_mapping() {
    let android_archs = vec![AndroidArch::ARM64, AndroidArch::ARMv7, AndroidArch::X86_64];
    assert_eq!(android_archs.len(), 3);
    assert!(matches!(android_archs[0], AndroidArch::ARM64));

    let ios_archs = vec![IOSArch::ARM64, IOSArch::ARM64e, IOSArch::X86_64];
    assert_eq!(ios_archs.len(), 3);
    assert!(matches!(ios_archs[0], IOSArch::ARM64));
}

#[test]
fn test_mobile_project_scaffolding() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_mobile_test_scaffold_{}", pid));
    let project_name = "DemoWidyaApp";

    let res = inisialisasi_proyek_mobile(project_name, Some(&temp_dir));
    assert!(res.is_ok());

    let proj_path = res.unwrap();
    assert!(proj_path.exists());
    assert!(proj_path.join("src").join("main.wya").exists());
    assert!(proj_path.join("widya.json").exists());
    assert!(proj_path.join("android").join("build.gradle").exists());
    assert!(proj_path.join("android").join("app").join("src").join("main").join("AndroidManifest.xml").exists());
    assert!(proj_path.join("ios").join(project_name).join("Info.plist").exists());
    assert!(proj_path.join("ios").join(project_name).join("ViewController.swift").exists());

    // Clean up
    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_mobile_html_rendering_pipeline() {
    let pid = std::process::id();
    let temp_wya = std::env::temp_dir().join(format!("test_mobile_render_{}.wya", pid));
    let code = r#"
        fungsi utama() {
            kembalikan Aplikasi({
                "judul": "Aplikasi Tes",
                "badan": Halaman({
                    "badan": Kolom([
                        TeksWidget("Halo Mobile!"),
                        Tombol("Tekan", fungsi() {})
                    ])
                })
            });
        }
    "#;
    std::fs::write(&temp_wya, code).unwrap();

    let compiler = MobileCompiler::default_compiler();
    let html_res = compiler.compile_to_mobile_html(&temp_wya);
    assert!(html_res.is_ok());

    let html = html_res.unwrap();
    assert!(html.contains("Aplikasi Tes"));
    assert!(html.contains("Halo Mobile!"));
    assert!(html.contains("widya-appbar"));
    assert!(html.contains("widya-tombol"));

    let _ = std::fs::remove_file(&temp_wya);
}

#[test]
fn test_mobile_build_android_package() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_mobile_build_test_android_{}", pid));
    let example_path = Path::new("contoh/54_aplikasi_mobile.wya");

    if example_path.exists() {
        let res = bangun_aplikasi_mobile(example_path, Some(&temp_dir), "android");
        assert!(res.is_ok());
        let apk_path = res.unwrap();
        assert!(apk_path.exists());
        assert_eq!(apk_path.extension().unwrap(), "apk");
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_mobile_build_ios_package() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_mobile_build_test_ios_{}", pid));
    let example_path = Path::new("contoh/54_aplikasi_mobile.wya");

    if example_path.exists() {
        let res = bangun_aplikasi_mobile(example_path, Some(&temp_dir), "ios");
        assert!(res.is_ok());
        let ipa_path = res.unwrap();
        assert!(ipa_path.exists());
        assert_eq!(ipa_path.extension().unwrap(), "ipa");
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_mobile_build_webview_package() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_mobile_build_test_webview_{}", pid));
    let example_path = Path::new("contoh/54_aplikasi_mobile.wya");

    if example_path.exists() {
        let res = bangun_aplikasi_mobile(example_path, Some(&temp_dir), "webview");
        assert!(res.is_ok());
        let html_path = res.unwrap();
        assert!(html_path.exists());
        assert_eq!(html_path.file_name().unwrap(), "index.html");
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}
