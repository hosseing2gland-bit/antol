#!/bin/bash
# اسکریپت باز کردن پورت 3000 در Azure NSG

SERVER_IP="108.143.173.222"
SERVER_USER="berellian"
SERVER_PASS="ABCDqwer1234"

echo "🔍 در حال جستجوی اطلاعات Azure NSG..."

# دریافت اطلاعات VM
sshpass -p "$SERVER_PASS" ssh -o ConnectTimeout=15 $SERVER_USER@$SERVER_IP << 'ENDSSH'
# بررسی اطلاعات Azure Metadata
echo "📋 اطلاعات VM از Azure Metadata:"
curl -H Metadata:true --connect-timeout 5 "http://169.254.169.254/metadata/instance?api-version=2021-02-01" 2>/dev/null | python3 -m json.tool 2>/dev/null | grep -E "name|resourceGroupName|subscriptionId" | head -10

echo ""
echo "🔧 برای باز کردن پورت، از یکی از روش‌های زیر استفاده کنید:"
echo ""
echo "=== روش 1: Azure Portal (ساده‌ترین) ==="
echo "1. به https://portal.azure.com بروید"
echo "2. به Virtual Machines بروید و VM خود را انتخاب کنید"
echo "3. از منو: Networking → Add inbound port rule"
echo "4. تنظیمات:"
echo "   - Destination port ranges: 3000"
echo "   - Protocol: TCP"
echo "   - Action: Allow"
echo "   - Priority: 1000"
echo "   - Name: Allow-Backend-3000"
echo "5. روی Add کلیک کنید"
echo ""
echo "=== روش 2: Azure CLI (از سرور) ==="
echo "# نصب Azure CLI:"
echo "curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash"
echo ""
echo "# ورود به Azure:"
echo "az login"
echo ""
echo "# باز کردن پورت:"
echo "az network nsg rule create \\"
echo "  --resource-group <YOUR_RESOURCE_GROUP> \\"
echo "  --nsg-name <YOUR_NSG_NAME> \\"
echo "  --name Allow-Backend-3000 \\"
echo "  --priority 1000 \\"
echo "  --source-address-prefixes '*' \\"
echo "  --source-port-ranges '*' \\"
echo "  --destination-address-prefixes '*' \\"
echo "  --destination-port-ranges 3000 \\"
echo "  --access Allow \\"
echo "  --protocol Tcp \\"
echo "  --description 'Allow Anti-Detect Backend API'"
echo ""
echo "=== روش 3: باز کردن موقت با UFW (برای تست) ==="
echo "sudo ufw allow 3000/tcp"
echo "sudo ufw enable"
echo ""

ENDSSH

echo "✅ راهنما نمایش داده شد."
echo ""
echo "💡 پیشنهاد: ساده‌ترین راه استفاده از Azure Portal است (روش 1)"
