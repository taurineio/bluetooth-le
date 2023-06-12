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
