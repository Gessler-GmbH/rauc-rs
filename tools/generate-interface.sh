#!/usr/bin/env bash
set -euo pipefail

mkdir -p src/generated

zbus-xmlgen file interfaces/de.pengutronix.rauc.Installer.xml -o src/generated/installer.rs
zbus-xmlgen file interfaces/de.pengutronix.rauc.Poller.xml -o src/generated/poller.rs