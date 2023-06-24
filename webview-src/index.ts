import {invoke} from '@tauri-apps/api/tauri';
import {listen} from '@tauri-apps/api/event';
import {from, Observable, Subject} from 'rxjs';
import {switchMap} from 'rxjs/operators';
import {
  BluetoothLEDevice,
  ConnectRequest,
  GetServicesRequest,
  InitRequest,
  ScanRequest,
} from './models';

export * from './models';

const eventPrefix = 'plugin:taurine-bluetooth-le|';

export class TaurineBluetoothLE {
  static init(req?: InitRequest): Promise<void> {
    return invoke(`${eventPrefix}init`, {payload: req || {}});
  }

  static scan(request?: ScanRequest): Observable<BluetoothLEDevice> {
    return from(invoke(`${eventPrefix}start_scan`, {payload: request})).pipe(
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

  static connect(request: ConnectRequest): Promise<void> {
    return invoke<void>(`${eventPrefix}connect`, {payload: request});
  }

  static getServices(request: GetServicesRequest): Promise<unknown[]> {
    return invoke<unknown[]>(`${eventPrefix}get_services`, {payload: request});
  }
}
