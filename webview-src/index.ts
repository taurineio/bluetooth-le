import {invoke} from '@tauri-apps/api/tauri';
import {listen} from '@tauri-apps/api/event';
import {from, Observable, Subject} from 'rxjs';
import {switchMap} from 'rxjs/operators';
import {BluetoothLEDevice, InitRequest} from './models';

export * from './models';

const eventPrefix = 'plugin:taurine-bluetooth-le|';

export class TaurineBluetoothLE {
  static init(req?: InitRequest): Promise<void> {
    return invoke(`${eventPrefix}init_ble`, {payload: req || {}});
  }

  static scan(timeout = 30000): Observable<BluetoothLEDevice> {
    return from(invoke(`${eventPrefix}start_scan`, {payload: {timeout}})).pipe(
      switchMap(() => {
        const stream = new Subject<BluetoothLEDevice>();
        const discoverListener = listen<BluetoothLEDevice>(
          `ble_device_discovered`,
          (event) => stream.next(event.payload)
        );
        const stopListener = listen(`ble_stop_scan`, async () => {
          await discoverListener;
          await stopListener;
          stream.complete();
        });
        return stream;
      })
    );
  }
}
