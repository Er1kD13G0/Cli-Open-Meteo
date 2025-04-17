# 🌤️ Weather CLI

Um simples aplicativo de linha de comando (CLI) feito em Rust para consultar o clima atual usando a [API gratuita da Open-Meteo](https://open-meteo.com/).

## ✨ Funcionalidades

- Consulta o clima atual (temperatura, vento, etc.)
- Utiliza coordenadas geográficas (latitude e longitude)
- Rápido, simples e feito em Rust 🦀

---

## 🛠️ Instalação

1. Clone este repositório:

```bash
git clone https://github.com/Er1kD13G0/Cli-Open-Meteo/
cd weather-cli

Compile o projeto:
cargo build --release

Rode o programa:
cargo run -- --latitude 23.55 --longitude -46.63
´```
Resultado:

Consultando o clima para latitude -23.55 e longitude -46.63...
Clima atual:
Tempo: 2025-04-16T12:00
Temperatura: 20.2 °C
Velocidade do vento: 12.8 km/h
Direção do vento: 101°
Código do clima: 0


