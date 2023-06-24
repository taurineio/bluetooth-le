export interface BluetoothLEDevice {
  address: string;
  name?: string;
  uuids: string[];
}

export interface AndroidInitRequest {
  forLocation: boolean;
}

export interface InitRequest {
  android?: AndroidInitRequest;
}

export interface ScanFilter {
  services?: string[];
}

export interface ScanRequest {
  timeout?: number;
  filter?: ScanFilter;
}

export interface ConnectRequest {
  address: string;
  timeout?: number;
}

export interface GetServicesRequest {
  address: string;
}
