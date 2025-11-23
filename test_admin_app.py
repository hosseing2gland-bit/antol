#!/usr/bin/env python3
"""
Admin App Structure Test Script
Tests the structure and React components of the admin app
Run from repository root: python test_admin_app.py
"""

import os
import sys

# Get the directory where this script is located (repository root)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

print("\n🎯 Admin App Structure Test")
print("================================\n")

# Test 1: Check main files
print("📁 Test 1: Main Files")
main_rs = os.path.join(SCRIPT_DIR, "anti-detect-mvp", "admin-app", "src-tauri", "src", "main.rs")
main_tsx = os.path.join(SCRIPT_DIR, "anti-detect-mvp", "admin-app", "src", "main.tsx")
app_tsx = os.path.join(SCRIPT_DIR, "anti-detect-mvp", "admin-app", "src", "App.tsx")
components_dir = os.path.join(SCRIPT_DIR, "anti-detect-mvp", "admin-app", "src", "components")

files_found = 0
for file in [main_rs, main_tsx, app_tsx]:
    if os.path.exists(file):
        print(f"   ✅ {os.path.basename(file)}")
        files_found += 1
    else:
        print(f"   ❌ {os.path.basename(file)} - NOT FOUND")

# Test 2: Check React components
print("\n📦 Test 2: React Components")
components = [
    "Dashboard.tsx",
    "Licenses.tsx",
    "Login.tsx",
    "Profiles.tsx",
    "Proxies.tsx",
    "Settings.tsx",
    "Sidebar.tsx",
    "Users.tsx",
]

components_found = 0
for comp in components:
    path = os.path.join(components_dir, comp)
    if os.path.exists(path):
        print(f"   ✅ {comp}")
        components_found += 1
    else:
        print(f"   ❌ {comp} - NOT FOUND")

# Test 3: Check key functions in main.rs
print("\n🔍 Test 3: Rust Key Functions")
key_functions = ["main", "run"]
functions_found = 0

try:
    with open(main_rs, encoding='utf-8') as f:
        content = f.read()
        for func in key_functions:
            if func in content:
                print(f"   ✅ {func} found in main.rs")
                functions_found += 1
            else:
                print(f"   ❌ {func} NOT in main.rs")
except Exception as e:
    print(f"   ⚠️  main.rs - Cannot read: {e}")

# Final Summary
print("\n✨ Test Summary")
print("================================")
print(f"Main files found:   {files_found}/3")
print(f"Components found:   {components_found}/{len(components)}")
print(f"Functions found:    {functions_found}/{len(key_functions)}")

if files_found >= 2 and components_found >= 5:
    print("\n🚀 Admin App Status: READY\n")
    sys.exit(0)
else:
    print("\n⚠️  Admin App Status: INCOMPLETE\n")
    sys.exit(1)
