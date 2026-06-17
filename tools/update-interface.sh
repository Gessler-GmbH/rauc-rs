#!/usr/bin/env bash
# This script updates the committed and official RAUC D-Bus interface XML.

set -euo pipefail

RAUC_VERSION="v1.15.2"

RAUC_REPOSITORY="https://raw.githubusercontent.com/rauc/rauc/${RAUC_VERSION}/src"

INTERFACES_DIRECTORY="interfaces"

INSTALLER_INTERFACE="de.pengutronix.rauc.Installer.xml"
# POLLER_INTERFACE="de.pengutronix.rauc.Poller.xml"

curl -L "${RAUC_REPOSITORY}/${INSTALLER_INTERFACE}" -o "${INTERFACES_DIRECTORY}/${INSTALLER_INTERFACE}"
# curl -L "${RAUC_REPOSITORY}/${POLLER_INTERFACE}" -o "${INTERFACES_DIRECTORY}/${POLLER_INTERFACE}"