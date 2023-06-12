<script lang="ts">
  import type {BluetoothLEDevice} from 'taurine-bluetooth-le-api';
  import {TaurineBluetoothLE} from 'taurine-bluetooth-le-api';

  let bleInit = false;
  let scanDisabled = false;
  let errorMessage = "";

  let devices = [] as BluetoothLEDevice[];

  const initBle = async () => {
    try {
      await TaurineBluetoothLE.init({android: {forLocation: false}});
    } catch (e) {
      errorMessage = "Bluetooth not available";
      scanDisabled = true;
    }
    bleInit = true;
  };
  const scanBle = async () => {
    if (!bleInit) {
      await initBle();
    }
    if (scanDisabled) {
      return;
    }
    scanDisabled = true;
    devices = [];
    TaurineBluetoothLE.scan().subscribe({
      next: device => devices = [...devices, device],
      error: e => {
        errorMessage = `${e}`;
        scanDisabled = false;
      },
      complete: () => scanDisabled = false,
    });
  };
</script>

<main class="container">
  <h1>Taurine Bluetooth LE Plugin</h1>

  <div>
    <button disabled={scanDisabled} on:click="{() => scanBle()}">Scan devices</button>
  </div>
  <div class="row">{errorMessage}</div>

  {#each devices as device}
  <div class="row">
    Device {device.address} ({device.name || 'unknown'}): {device.uuids.length === 0 ? 'no service' : device.uuids.join(', ')}
  </div>
  {/each}
</main>
