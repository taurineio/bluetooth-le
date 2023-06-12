import { Observable } from 'rxjs';
import { BluetoothLEDevice, InitRequest } from './models';
export * from './models';
export declare class TaurineBluetoothLE {
    static init(req?: InitRequest): Promise<void>;
    static scan(timeout?: number): Observable<BluetoothLEDevice>;
}
