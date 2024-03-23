import {invoke} from '@tauri-apps/api/tauri'


export const scanSensors = async (): Promise<string[]> => {
    return await invoke('scan_sensors')
}


export const start = async (data: { payload: { sensor: string, mode: string } }): Promise<string[]> => {
    return await invoke('start', data)
}

export const stop = async (): Promise<string[]> => {
    return await invoke('stop')
}