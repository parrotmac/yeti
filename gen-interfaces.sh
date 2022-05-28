#!/usr/bin/env bash

dbus-codegen-rust < org.freedesktop.ModemManager1.Bearer.0.xml > bearer.rs
dbus-codegen-rust < org.freedesktop.ModemManager1.Modem.0.xml > modem.rs
dbus-codegen-rust < org.freedesktop.ModemManager1.SIM.0.xml > sim.rs
dbus-codegen-rust < org.freedesktop.ModemManager1.SMS.0.xml > sms.rs
dbus-codegen-rust < org.freedesktop.ModemManager1.xml > modemmanager.rs
