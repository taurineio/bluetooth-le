package io.taurine.bluetooth_le.plugin

import android.Manifest
import android.app.Activity
import android.os.Build
import app.tauri.PermissionState
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.PermissionCallback
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin(
  permissions = [
    Permission(strings = [Manifest.permission.BLUETOOTH], alias = "BLUETOOTH"),
    Permission(strings = [Manifest.permission.BLUETOOTH_CONNECT], alias = "BLUETOOTH_CONNECT"),
    Permission(strings = [Manifest.permission.BLUETOOTH_SCAN], alias = "BLUETOOTH_SCAN"),
    Permission(strings = [Manifest.permission.ACCESS_FINE_LOCATION], alias = "ACCESS_COARSE_LOCATION"),
    Permission(strings = [Manifest.permission.ACCESS_FINE_LOCATION], alias = "ACCESS_FINE_LOCATION"),
  ]
)
class TaurineBluetoothLEPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = TaurineBluetoothLE()
    private var aliases: Array<String> = arrayOf()

    @Command
    fun initBle(invoke: Invoke) {
        val forLocation = invoke.getBoolean("forLocation") ?: false
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            aliases = arrayOf("BLUETOOTH_SCAN", "BLUETOOTH_CONNECT", "BLUETOOTH_ADMIN")
            if (forLocation) {
                aliases += "ACCESS_FINE_LOCATION"
            }
        } else {
            aliases = arrayOf("BLUETOOTH")
            if (Build.VERSION.SDK_INT > Build.VERSION_CODES.P) {
                aliases += "ACCESS_FINE_LOCATION"
            } else {
                aliases += "ACCESS_COARSE_LOCATION"
            }
        }
        requestPermissionForAliases(aliases, invoke, "permissionsCallback")
    }

    @PermissionCallback
    private fun permissionsCallback(invoke: Invoke) {
        val granted: List<Boolean> = aliases.map { alias ->
            getPermissionState(alias) == PermissionState.GRANTED
        }
        // all have to be true
        if (granted.all { it }) {
            invoke.resolve()
        } else {
            var notGranted: List<String> = aliases.filter {alias -> getPermissionState(alias) != PermissionState.GRANTED}
            invoke.reject("Missing permissions " + notGranted.joinToString())
        }
    }
}
