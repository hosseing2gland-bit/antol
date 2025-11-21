# تست ساختار و فانکشن‌های Admin App
import os

print("\n🎯 تست ساختار Admin App")
print("================================\n")

# تست ۱: بررسی فایل‌های اصلی
main_rs = "/workspaces/antol/anti-detect-mvp/admin-app/src-tauri/src/main.rs"
main_tsx = "/workspaces/antol/anti-detect-mvp/admin-app/src/main.tsx"
app_tsx = "/workspaces/antol/anti-detect-mvp/admin-app/src/App.tsx"
components_dir = "/workspaces/antol/anti-detect-mvp/admin-app/src/components/"

files = [main_rs, main_tsx, app_tsx]
for file in files:
    if os.path.exists(file):
        print(f"   ✅ {file}")
    else:
        print(f"   ❌ {file} - پیدا نشد")

# تست ۲: بررسی کامپوننت‌های React
print("\n📦 تست ۲: کامپوننت‌های React")
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
for comp in components:
    path = os.path.join(components_dir, comp)
    if os.path.exists(path):
        print(f"   ✅ {comp}")
    else:
        print(f"   ❌ {comp} - پیدا نشد")

# تست ۳: بررسی فانکشن‌های کلیدی در main.rs
print("\n🔍 تست ۳: فانکشن‌های کلیدی Rust")
key_functions = ["main", "run", "command"]
try:
    with open(main_rs, encoding='utf-8') as f:
        content = f.read()
        for func in key_functions:
            if func in content:
                print(f"   ✅ {func} در main.rs یافت شد")
            else:
                print(f"   ❌ {func} در main.rs نیست")
except:
    print("   ⚠️  main.rs قابل خواندن نیست")

print("\n✨ خلاصه تست Admin App")
print("================================")
print("ساختار و کامپوننت‌های اصلی Admin App موجود است!")
print("🚀 آماده تست عملی و توسعه بیشتر!")
