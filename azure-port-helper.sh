#!/bin/bash
# اسکریپت اتوماتیک باز کردن پورت 3000 در Azure

echo "🚀 شروع باز کردن پورت 3000 در Azure NSG..."

# اطلاعات Azure (از metadata دریافت شده)
RESOURCE_GROUP="keke_group"
VM_NAME="keke"
NSG_NAME="${VM_NAME}-nsg"  # معمولاً NSG به همین شکل نام‌گذاری می‌شود

echo "📋 اطلاعات:"
echo "   Resource Group: $RESOURCE_GROUP"
echo "   VM Name: $VM_NAME"
echo "   NSG Name (حدسی): $NSG_NAME"
echo ""

# دستورات برای اجرا در Azure Cloud Shell یا Azure CLI
cat << 'EOF' > /tmp/azure-open-port.sh
#!/bin/bash
# این اسکریپت را در Azure Cloud Shell اجرا کنید

RESOURCE_GROUP="keke_group"

# پیدا کردن NSG
echo "🔍 در حال جستجوی NSG..."
NSG_NAME=$(az network nsg list --resource-group $RESOURCE_GROUP --query "[0].name" -o tsv)

if [ -z "$NSG_NAME" ]; then
    echo "❌ NSG پیدا نشد!"
    exit 1
fi

echo "✅ NSG پیدا شد: $NSG_NAME"
echo ""

# باز کردن پورت 3000
echo "🔓 باز کردن پورت 3000..."
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $NSG_NAME \
  --name Allow-Backend-3000 \
  --priority 1000 \
  --source-address-prefixes '*' \
  --source-port-ranges '*' \
  --destination-address-prefixes '*' \
  --destination-port-ranges 3000 \
  --access Allow \
  --protocol Tcp \
  --description 'Allow Anti-Detect Backend API on port 3000'

if [ $? -eq 0 ]; then
    echo "✅ پورت 3000 با موفقیت باز شد!"
    echo ""
    echo "🧪 تست اتصال:"
    echo "   curl http://108.143.173.222:3000/api"
else
    echo "❌ خطا در باز کردن پورت"
fi
EOF

chmod +x /tmp/azure-open-port.sh

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 سه روش برای باز کردن پورت:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "### روش 1️⃣: Azure Portal (ساده‌ترین - پیشنهاد می‌شود)"
echo ""
echo "1. به https://portal.azure.com بروید"
echo "2. در Search بالا: 'keke' تایپ کنید و VM را انتخاب کنید"
echo "3. از منوی سمت چپ: Settings → Networking"
echo "4. روی 'Add inbound port rule' کلیک کنید"
echo "5. فرم را پر کنید:"
echo "   ┌─────────────────────────────────┐"
echo "   │ Source: Any                     │"
echo "   │ Source port ranges: *           │"
echo "   │ Destination: Any                │"
echo "   │ Service: Custom                 │"
echo "   │ Destination port ranges: 3000   │"
echo "   │ Protocol: TCP                   │"
echo "   │ Action: Allow                   │"
echo "   │ Priority: 1000                  │"
echo "   │ Name: Allow-Backend-3000        │"
echo "   └─────────────────────────────────┘"
echo "6. روی 'Add' کلیک کنید"
echo "7. صبر کنید تا rule اعمال شود (30-60 ثانیه)"
echo ""
echo "### روش 2️⃣: Azure Cloud Shell"
echo ""
echo "1. به https://portal.azure.com بروید"
echo "2. روی آیکون Cloud Shell کلیک کنید (بالای صفحه، کنار search)"
echo "3. این دستور را کپی و اجرا کنید:"
echo ""
echo "az network nsg rule create \\"
echo "  --resource-group keke_group \\"
echo "  --nsg-name keke-nsg \\"
echo "  --name Allow-Backend-3000 \\"
echo "  --priority 1000 \\"
echo "  --source-address-prefixes '*' \\"
echo "  --destination-port-ranges 3000 \\"
echo "  --access Allow \\"
echo "  --protocol Tcp"
echo ""
echo "### روش 3️⃣: Azure CLI از سرور (پیچیده‌تر)"
echo ""
echo "اسکریپت آماده در: /tmp/azure-open-port.sh"
echo "برای اجرا:"
echo "  scp berellian@108.143.173.222:/tmp/azure-open-port.sh ."
echo "  # سپس در Azure Cloud Shell اجرا کنید"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ بعد از باز کردن پورت، با این دستور تست کنید:"
echo "   curl http://108.143.173.222:3000/api"
echo ""
echo "📧 اگر مشکلی بود، لاگ backend را چک کنید:"
echo "   ssh berellian@108.143.173.222 'tail -50 ~/antol/anti-detect-mvp/backend.log'"
echo ""
