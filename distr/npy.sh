#!/bin/bash

# ==========================================
# npy Interactive CLI Shell Launcher (Linux)
# ==========================================

# Pastikan script dijalankan dari directory tempat file ini berada
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXE="$SCRIPT_DIR/npy"

clear

# Warna cyan (opsional)
CYAN="\033[36m"
RESET="\033[0m"

echo -e "${CYAN}"
echo " ███╗   ██╗██████╗ ██╗   ██╗"
echo " ████╗  ██║██╔══██╗╚██╗ ██╔╝"
echo " ██╔██╗ ██║██████╔╝ ╚████╔╝ "
echo " ██║╚██╗██║██╔═══╝   ╚██╔╝  "
echo " ██║ ╚████║██║        ██║   "
echo " ╚═╝  ╚═══╝╚═╝        ╚═╝   "
echo "--------------------------------------------------------------"
echo "    Data pipeline and scraper for network pharmacology"
echo "                          by Yumana"
echo "--------------------------------------------------------------"
echo -e "${RESET}\n"

# Cek apakah binary ada
if [[ ! -f "$EXE" ]]; then
  echo "[ERROR] npy tidak ditemukan!"
  echo "Pastikan file ada di: $SCRIPT_DIR"
  exit 1
fi

echo "Memulai NPY CLI..."
echo

# Jika terminal "cool" tersedia, seperti gnome-terminal atau konsole
if command -v gnome-terminal &>/dev/null; then
  gnome-terminal -- bash -c "\"$EXE\" --help; exec bash"
  exit 0
elif command -v konsole &>/dev/null; then
  konsole -e bash -c "\"$EXE\" --help; exec bash"
  exit 0
elif command -v xterm &>/dev/null; then
  xterm -e "$EXE --help"
  exit 0
fi

# FALLBACK → jalankan langsung di terminal saat ini
"$EXE" --help
