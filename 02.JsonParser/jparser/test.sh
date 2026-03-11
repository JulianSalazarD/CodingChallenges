#!/usr/bin/fish

# Imprime un mensaje con color para saber qué está pasando
set_color cyan
echo "🚀 Iniciando ejecución múltiple de Cargo..."
set_color normal

# Step 1
cargo run -- tests/step1/invalid.json
cargo run -- tests/step1/valid.json

# Step 2
cargo run -- tests/step2/invalid.json
cargo run -- tests/step2/invalid2.json
cargo run -- tests/step2/valid.json
cargo run -- tests/step2/valid2.json

# Step3
cargo run -- tests/step3/invalid.json
cargo run -- tests/step3/valid.json

# step4
cargo run -- tests/step4/invalid.json
cargo run -- tests/step4/valid.json
cargo run -- tests/step4/valid2.json


set_color green
echo "✅ Todas las ejecuciones han finalizado."
set_color normal
