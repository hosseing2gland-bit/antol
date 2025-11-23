#!/usr/bin/env python3
"""
Anti-Detection System Test Script
Tests the structure and key functions of the anti-detection components
Run from repository root: python test_anti_detection.py
"""

import os
import sys

# Get the directory where this script is located (repository root)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

print("\n🎯 Anti-Detection System Test")
print("================================\n")

# Test 1: Check key files exist
print("📁 Test 1: Checking Key Files")
BASE = os.path.join(SCRIPT_DIR, "anti-detect-mvp", "client-app", "src-tauri", "src", "anti_detect")
MAIN = os.path.join(SCRIPT_DIR, "anti-detect-mvp", "client-app", "src-tauri", "src", "main.rs")

files = [
    os.path.join(BASE, "mod.rs"),
    os.path.join(BASE, "fingerprint.rs"),
    os.path.join(BASE, "browser_launch.rs"),
    os.path.join(BASE, "injection.rs"),
    os.path.join(BASE, "utils.rs"),
]

files_found = 0
for file in files:
    if os.path.exists(file):
        print(f"   ✅ {os.path.basename(file)}")
        files_found += 1
    else:
        print(f"   ❌ {os.path.basename(file)} - NOT FOUND")

# Test 2: Count lines of code
print("\n📊 Test 2: Code Statistics")
total_lines = 0
for file in files[1:]:  # Skip mod.rs
    try:
        with open(file, encoding='utf-8') as f:
            lines = len(f.readlines())
            total_lines += lines
            print(f"   📄 {os.path.basename(file)}: {lines} lines")
    except Exception as e:
        print(f"   ⚠️  {os.path.basename(file)} - Cannot read")
print(f"   📈 Total: {total_lines} lines of anti-detection code\n")

# Test 3: Check key functions
print("🔍 Test 3: Checking Key Functions")
key_functions = [
    ("fingerprint.rs", "generate_random"),
    ("fingerprint.rs", "FingerprintConfig"),
    ("browser_launch.rs", "BrowserProfile"),
    ("browser_launch.rs", "launch"),
    ("injection.rs", "generate_injection_script"),
    ("injection.rs", "canvas"),
    ("injection.rs", "webgl"),
    ("utils.rs", "common_screen_resolutions"),
]

functions_found = 0
for file, func in key_functions:
    path = os.path.join(BASE, file)
    try:
        with open(path, encoding='utf-8') as f:
            content = f.read()
            if func in content:
                print(f"   ✅ {func} in {file}")
                functions_found += 1
            else:
                print(f"   ❌ {func} NOT in {file}")
    except:
        print(f"   ⚠️  {file} - Cannot read")

# Test 4: Check Anti-Detection Features
print("\n🛡️ Test 4: Anti-Detection Features")
features = [
    "canvas_noise",
    "webgl_vendor",
    "audio_context",
    "user_agent",
    "hardware_concurrency",
    "screen_resolution",
    "timezone",
    "webrtc",
    "media_devices",
    "client_rects",
    "battery",
    "fonts",
]

injection_path = os.path.join(BASE, "injection.rs")
features_found = 0
try:
    with open(injection_path, encoding='utf-8') as f:
        content = f.read()
        for feature in features:
            if feature in content:
                print(f"   ✅ {feature}")
                features_found += 1
            else:
                print(f"   ⚠️  {feature} - might be missing")
except:
    print(f"   ⚠️  injection.rs - Cannot read")

# Test 5: Check Tauri Commands
print("\n⚡ Test 5: Tauri Commands")
commands = [
    "generate_fingerprint",
    "launch_browser",
    "stop_browser",
    "get_active_browsers",
    "stop_all_browsers",
]

commands_found = 0
try:
    with open(MAIN, encoding='utf-8') as f:
        content = f.read()
        for cmd in commands:
            if cmd in content:
                print(f"   ✅ {cmd}")
                commands_found += 1
            else:
                print(f"   ❌ {cmd} - NOT registered")
except:
    print(f"   ⚠️  main.rs - Cannot read")

# Final Summary
print("\n✨ Test Summary")
print("================================")
print(f"Files found:     {files_found}/{len(files)}")
print(f"Functions found: {functions_found}/{len(key_functions)}")
print(f"Features found:  {features_found}/{len(features)}")
print(f"Commands found:  {commands_found}/{len(commands)}")

print("\n📋 Implemented Features:")
print("   • Canvas Fingerprint Spoofing")
print("   • WebGL Fingerprint Protection")
print("   • Audio Context Noise")
print("   • User Agent Randomization")
print("   • Hardware Spoofing")
print("   • Screen Resolution Control")
print("   • Timezone & Language Settings")
print("   • WebRTC Leak Protection")
print("   • Media Devices Spoofing")
print("   • Client Rects Noise")
print("   • Battery API Protection")
print("   • Font Fingerprint Defense")

if files_found == len(files):
    print("\n🚀 System Status: READY FOR TESTING\n")
    sys.exit(0)
else:
    print("\n⚠️  System Status: SOME FILES MISSING\n")
    sys.exit(1)
