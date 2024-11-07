cargo build --release --target wasm32-unknown-unknown --package http_outcalls_backend

candid-extractor target/wasm32-unknown-unknown/release/http_outcalls_backend.wasm > src/http_outcalls_backend/http_outcalls_backend.did


# curl -X POST "https://api.telegram.org/bot8063785097:AAEhj5UIXY9e_uOgxtL-uXIkkbi1IYF98Cw/setWebhook?url=https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=3s2f2-niaaa-aaaam-adqwa-cai/receive_telegram_message"


# admin@admins-MacBook-Air ~ % curl -X POST "https://api.telegram.org/bot8063785097:AAEhj5UIXY9e_uOgxtL-uXIkkbi1IYF98Cw/setWebhook?url=https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=3s2f2-niaaa-aaaam-adqwa-cai/receive_telegram_message"

# {"ok":true,"result":true,"description":"Webhook was set"}%                                                                                                                                                  admin@admins-MacBook-Air ~ % curl -X GET "https://api.telegram.org/bot8063785097:AAEhj5UIXY9e_uOgxtL-uXIkkbi1IYF98Cw/getWebhookInfo"

# {"ok":true,"result":{"url":"https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=3s2f2-niaaa-aaaam-adqwa-cai/receive_telegram_message","has_custom_certificate":false,"pending_update_count":0,"max_connections":40,"ip_address":"212.71.124.189"}}%   