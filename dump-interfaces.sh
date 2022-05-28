#!/usr/bin/env bash

set -euo pipefail

dbus-send --system \
        --type=method_call \
        --print-reply=literal \
        --dest=org.freedesktop.ModemManager1 \
        /org/freedesktop/ModemManager1 \
        org.freedesktop.DBus.Introspectable.Introspect \
        > org.freedesktop.ModemManager1.xml

dbus-send --system \
        --type=method_call \
        --print-reply=literal \
        --dest=org.freedesktop.ModemManager1 \
        /org/freedesktop/ModemManager1/Modem/0 \
        org.freedesktop.DBus.Introspectable.Introspect \
        > org.freedesktop.ModemManager1.Modem.0.xml

dbus-send --system \
        --type=method_call \
        --print-reply=literal \
        --dest=org.freedesktop.ModemManager1 \
        /org/freedesktop/ModemManager1/Bearer/0 \
        org.freedesktop.DBus.Introspectable.Introspect \
        > org.freedesktop.ModemManager1.Bearer.0.xml

dbus-send --system \
        --type=method_call \
        --print-reply=literal \
        --dest=org.freedesktop.ModemManager1 \
        /org/freedesktop/ModemManager1/SIM/0 \
        org.freedesktop.DBus.Introspectable.Introspect \
        > org.freedesktop.ModemManager1.SIM.0.xml

dbus-send --system \
        --type=method_call \
        --print-reply=literal \
        --dest=org.freedesktop.ModemManager1 \
        /org/freedesktop/ModemManager1/SMS/0 \
        org.freedesktop.DBus.Introspectable.Introspect \
        > org.freedesktop.ModemManager1.SMS.0.xml
