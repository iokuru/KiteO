import json
import os
import shutil
import subprocess
import zipfile

EXTENSION_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

MATCHES = [
    "*://codeforces.com/*",
    "*://leetcode.com/*",
    "*://www.codechef.com/*",
    "*://atcoder.jp/*",
    "*://www.hackerrank.com/*",
    "*://www.geeksforgeeks.org/*",
    "*://www.hackerearth.com/*",
    "*://cses.fi/*",
    "*://www.onlinegdb.com/*",
    "*://www.jdoodle.com/*",
    "*://www.programiz.com/*",
    "*://replit.com/*",
]

def zip_directory(source_dir, output_zip):
    with zipfile.ZipFile(output_zip, "w", zipfile.ZIP_DEFLATED) as zf:
        for root, _, files in os.walk(source_dir):
            for file in files:
                full_path = os.path.join(root, file)
                rel_path = os.path.relpath(full_path, source_dir)
                zf.write(full_path, rel_path)
    size_kb = os.path.getsize(output_zip) / 1024.0
    print(f"Created archive: {output_zip} ({size_kb:.1f} KB)")

def main():
    print("=== Building Kite0 Browser Extension ===")
    subprocess.run(["npm", "run", "build"], cwd=EXTENSION_ROOT, check=True, shell=True)

    dist_dir = os.path.join(EXTENSION_ROOT, "dist")
    packages_dir = os.path.join(EXTENSION_ROOT, "dist-packages")
    os.makedirs(packages_dir, exist_ok=True)

    icons_src = os.path.join(EXTENSION_ROOT, "public", "icons")

    # 1. Chrome MV3
    print("\n--- Packaging Chrome (Manifest V3) ---")
    chrome_dir = os.path.join(EXTENSION_ROOT, "dist-chrome")
    if os.path.exists(chrome_dir):
        shutil.rmtree(chrome_dir)
    shutil.copytree(dist_dir, chrome_dir)
    shutil.copytree(icons_src, os.path.join(chrome_dir, "icons"), dirs_exist_ok=True)

    chrome_manifest = {
        "manifest_version": 3,
        "name": "Kite0 - Static Complexity Analyzer",
        "version": "0.1.0",
        "description": "Deterministic local static complexity analyzer for competitive programming. Zero telemetry, 100% offline WebAssembly.",
        "icons": {
            "16": "icons/icon-16.png",
            "48": "icons/icon-48.png",
            "128": "icons/icon-128.png",
        },
        "permissions": ["activeTab"],
        "background": {
            "service_worker": "background.js",
            "type": "module",
        },
        "action": {
            "default_popup": "src/popup/index.html",
            "default_title": "Kite0 Complexity Analyzer",
            "default_icon": {
                "16": "icons/icon-16.png",
                "48": "icons/icon-48.png",
                "128": "icons/icon-128.png",
            },
        },
        "content_scripts": [
            {
                "matches": MATCHES,
                "js": ["content.js"],
            }
        ],
    }

    with open(os.path.join(chrome_dir, "manifest.json"), "w", encoding="utf-8") as f:
        json.dump(chrome_manifest, f, indent=2)

    chrome_zip = os.path.join(packages_dir, "kiteo-chrome-v0.1.0.zip")
    zip_directory(chrome_dir, chrome_zip)

    # 2. Firefox (Gecko MV3 / Scripts background)
    print("\n--- Packaging Firefox ---")
    firefox_dir = os.path.join(EXTENSION_ROOT, "dist-firefox")
    if os.path.exists(firefox_dir):
        shutil.rmtree(firefox_dir)
    shutil.copytree(dist_dir, firefox_dir)
    shutil.copytree(icons_src, os.path.join(firefox_dir, "icons"), dirs_exist_ok=True)

    firefox_manifest = {
        "manifest_version": 3,
        "name": "Kite0 - Static Complexity Analyzer",
        "version": "0.1.0",
        "description": "Deterministic local static complexity analyzer for competitive programming. Zero telemetry, 100% offline WebAssembly.",
        "browser_specific_settings": {
            "gecko": {
                "id": "kiteo@iokuru.github.io",
                "strict_min_version": "109.0",
            }
        },
        "icons": {
            "16": "icons/icon-16.png",
            "48": "icons/icon-48.png",
            "128": "icons/icon-128.png",
        },
        "permissions": ["activeTab"],
        "background": {
            "scripts": ["background.js"],
        },
        "action": {
            "default_popup": "src/popup/index.html",
            "default_title": "Kite0 Complexity Analyzer",
            "default_icon": {
                "16": "icons/icon-16.png",
                "48": "icons/icon-48.png",
                "128": "icons/icon-128.png",
            },
        },
        "content_scripts": [
            {
                "matches": MATCHES,
                "js": ["content.js"],
            }
        ],
    }

    with open(os.path.join(firefox_dir, "manifest.json"), "w", encoding="utf-8") as f:
        json.dump(firefox_manifest, f, indent=2)

    firefox_zip = os.path.join(packages_dir, "kiteo-firefox-v0.1.0.zip")
    zip_directory(firefox_dir, firefox_zip)

    print("\n=== Extension Packaging Complete ===")
    print(f"Chrome distribution:  {chrome_dir} -> {chrome_zip}")
    print(f"Firefox distribution: {firefox_dir} -> {firefox_zip}")

if __name__ == "__main__":
    main()
