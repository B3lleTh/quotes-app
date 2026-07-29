# Quotes // Paiheme

App de escritorio (macOS) para guardar tus frases/citas favoritas de libros o de donde sea, verlas al azar, y compararlas entre sí con un sistema de ranking tipo **ELO** para descubrir cuáles son realmente tus favoritas con el tiempo.

100% local — usa una base de datos SQLite en tu computadora, sin internet ni cuentas.

## Features

- **Random**: te muestra una frase al azar de tu colección.
- **Duel**: te enseña 2 frases, eliges cuál te gusta más → ajusta el rating ELO de ambas.
- **Top**: ranking completo ordenado por ELO.
- **Agregar**: escribe o pega frases + fuente opcional (libro/autor). No permite duplicados.
- **Backup**: exporta tu colección a JSON con un click.
- Confirmación antes de borrar, toasts de feedback, todo animado.

## Requisitos

- macOS
- [Rust](https://www.rust-lang.org/tools/install) (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- [Node.js](https://nodejs.org) (v18+)

## Instalación y correr en modo desarrollo

```bash
git clone https://github.com/TU-USUARIO/quotes-app.git
cd quotes-app
npm install
npm run tauri dev
```

## Compilar como app nativa (.app / .dmg)

```bash
npm run tauri build
```

El resultado queda en:

```
src-tauri/target/release/bundle/macos/Quotes.app
src-tauri/target/release/bundle/dmg/Quotes_0.1.0_aarch64.dmg
```

Arrastra `Quotes.app` a tu carpeta **Aplicaciones** y ábrela como cualquier app normal.

> **Nota:** como no está firmada con una cuenta de Apple Developer, la primera vez macOS dirá que es de "desarrollador no identificado". Solución: clic derecho sobre la app → **Abrir** → **Abrir de todos modos**.

## Dónde vive tu información

```
~/Library/Application Support/quotes-app/quotes.db       ← tu base de datos
~/Library/Application Support/quotes-app/backups/        ← backups en JSON
```

Puedes copiar esa carpeta a otra Mac para migrar tu colección completa.

## Estructura del proyecto

```
src/                    → frontend (HTML/CSS/JS, sin frameworks)
src-tauri/src/
  main.rs               → arranque de la app
  db.rs                 → conexión y esquema de SQLite
  models.rs             → struct Quote
  commands.rs           → CRUD + lógica de ELO
  backup.rs             → exportar/importar JSON
```

## Stack

Tauri v2 + Rust (rusqlite) + SQLite + HTML/CSS/JS vanilla.
