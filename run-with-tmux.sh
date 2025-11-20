#!/bin/bash

# رنگ‌ها
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}=== اجرای پروژه با tmux ===${NC}\n"

# بررسی tmux
if ! command -v tmux &> /dev/null; then
    echo -e "${YELLOW}tmux نصب نیست. در حال نصب...${NC}"
    brew install tmux
fi

# اگر session قبلی وجود دارد، حذف کن
tmux has-session -t antidetect 2>/dev/null
if [ $? == 0 ]; then
    echo -e "${YELLOW}Session قبلی پیدا شد. در حال حذف...${NC}"
    tmux kill-session -t antidetect
fi

# تشخیص معماری
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-apple-darwin"
else
    TARGET="x86_64-apple-darwin"
fi

echo -e "${GREEN}ایجاد session جدید...${NC}"

# ایجاد session جدید و window اول
tmux new-session -d -s antidetect -n 'Backend'

# Window 0: Backend
echo -e "${GREEN}راه‌اندازی Backend...${NC}"
tmux send-keys -t antidetect:0 'cd anti-detect-mvp/backend' C-m
tmux send-keys -t antidetect:0 'echo "Starting Backend on http://localhost:8080..."' C-m
tmux send-keys -t antidetect:0 'cargo run --release' C-m

# Window 1: License Server
echo -e "${GREEN}راه‌اندازی License Server...${NC}"
tmux new-window -t antidetect:1 -n 'License'
tmux send-keys -t antidetect:1 'cd anti-detect-mvp/license-server' C-m
tmux send-keys -t antidetect:1 'echo "Starting License Server on http://localhost:8081..."' C-m
tmux send-keys -t antidetect:1 'cargo run --release' C-m

# Window 2: Admin App
echo -e "${GREEN}راه‌اندازی Admin App...${NC}"
tmux new-window -t antidetect:2 -n 'Admin'
tmux send-keys -t antidetect:2 'cd anti-detect-mvp/admin-app' C-m
tmux send-keys -t antidetect:2 'echo "Starting Admin App..."' C-m
tmux send-keys -t antidetect:2 'npm run tauri dev' C-m

# Window 3: Client App
echo -e "${GREEN}راه‌اندازی Client App...${NC}"
tmux new-window -t antidetect:3 -n 'Client'
tmux send-keys -t antidetect:3 'cd anti-detect-mvp/client-app' C-m
tmux send-keys -t antidetect:3 'echo "Starting Client App..."' C-m
tmux send-keys -t antidetect:3 'npm run tauri dev' C-m

# Window 4: Logs/Terminal
tmux new-window -t antidetect:4 -n 'Logs'
tmux send-keys -t antidetect:4 'echo "=== دستورات مفید tmux ==="' C-m
tmux send-keys -t antidetect:4 'echo "Ctrl+B سپس:"' C-m
tmux send-keys -t antidetect:4 'echo "  0-4: تغییر به window مورد نظر"' C-m
tmux send-keys -t antidetect:4 'echo "  D: جدا شدن از session (برنامه‌ها ادامه می‌یابند)"' C-m
tmux send-keys -t antidetect:4 'echo "  [: اسکرول کردن (q برای خروج)"' C-m
tmux send-keys -t antidetect:4 'echo "  ,: تغییر نام window"' C-m
tmux send-keys -t antidetect:4 'echo ""' C-m
tmux send-keys -t antidetect:4 'echo "برای اتصال مجدد: tmux attach -t antidetect"' C-m
tmux send-keys -t antidetect:4 'echo "برای متوقف کردن همه: tmux kill-session -t antidetect"' C-m
tmux send-keys -t antidetect:4 'echo ""' C-m
tmux send-keys -t antidetect:4 'echo "Services:"' C-m
tmux send-keys -t antidetect:4 'echo "  Backend:        http://localhost:8080"' C-m
tmux send-keys -t antidetect:4 'echo "  License Server: http://localhost:8081"' C-m

# برگشت به window اول
tmux select-window -t antidetect:0

echo -e "\n${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              تمام سرویس‌ها راه‌اندازی شدند! 🚀           ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${YELLOW}دستورات tmux:${NC}"
echo -e "  ${GREEN}Ctrl+B${NC} سپس ${GREEN}0-4${NC}  : تغییر بین window‌ها"
echo -e "  ${GREEN}Ctrl+B${NC} سپس ${GREEN}D${NC}    : جدا شدن (برنامه‌ها ادامه می‌یابند)"
echo -e "  ${GREEN}Ctrl+B${NC} سپس ${GREEN}[${NC}    : اسکرول (${GREEN}q${NC} برای خروج)"
echo -e ""
echo -e "${YELLOW}برای اتصال مجدد:${NC} tmux attach -t antidetect"
echo -e "${YELLOW}برای توقف:${NC} tmux kill-session -t antidetect"
echo -e ""

# اتصال به session
echo -e "${GREEN}در حال اتصال به session...${NC}\n"
sleep 2
tmux attach-target antidetect
