import { Observable } from 'rxjs';
import { BluetoothLEDevice, ConnectRequest, GetServicesRequest, InitRequest, ScanRequest } from './models';
export * from './models';
export declare class TaurineBluetoothLE {
    static init(req?: InitRequest): Promise<void>;
    static scan(request?: ScanRequest): Observable<BluetoothLEDevice>;
    static connect(request: ConnectRequest): Promise<void>;
    static getServices(request: GetServicesRequest): Promise<unknown[]>;
}
