<script lang="ts">
  import type {BluetoothLEDevice} from 'taurine-bluetooth-le-api';
  import {TaurineBluetoothLE} from 'taurine-bluetooth-le-api';

  let bleInit = false;
  let scanDisabled = false;
  let connectDisabled = false;
  let errorMessage = "";

  let devices = [] as BluetoothLEDevice[];
  let connectedDevices = [] as string[];

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
  const connect = async (device: BluetoothLEDevice) => {
    if (connectDisabled) {
      return;
    }
    connectDisabled = true;
    let connected = false;
    console.log(device);
    try {
      await TaurineBluetoothLE.connect(device);
      connected = true;
    } catch (e) {
      if (e === "Already connected") {
        connected = true;
      } else {
        console.log(e);
      }
    }
    if (connected) {
      connectedDevices = [...connectedDevices, device.address];
    }
    connectDisabled = false;
  };
  const showServices = async (device: BluetoothLEDevice) => {
    try {
      const services = await TaurineBluetoothLE.getServices(device);
      console.log(services);
    } catch (e) {
      console.log(e);
    }
  };
</script>

<main class="container">
  <h1>Taurine Bluetooth LE Plugin</h1>

  <div>
    <button disabled={scanDisabled} on:click={() => scanBle()}>Scan devices</button>
  </div>
  <div class="row">{errorMessage}</div>

  <div class="col container">
    {#each devices as device}
    <div class="row">
      Device {device.address} ({device.name || 'unknown'})
      {#if connectedDevices.includes(device.address)}
        <button on:click={() => showServices(device)}>Services</button>
        {:else}
        <button disabled={connectDisabled} on:click={() => connect(device)}>Connect</button>
      {/if}
    </div>
    {/each}
  </div>
</main>
