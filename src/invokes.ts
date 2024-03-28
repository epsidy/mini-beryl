import {invoke} from '@tauri-apps/api/tauri'


export const scanSensors = async (): Promise<string[]> => {
    return await invoke('scan_sensors')
}

export const start_normal_mode = async (data: { payload: { sensor: string, mode: string } }): Promise<string[]> => {
    return await invoke('start_normal_mode', data)
}

export const start_hall_mode = async (data: { payload: { sensor: string, mode: string } }): Promise<string[]> => {
    return await invoke('start_hall_mode', data)
}

export const stop = async (): Promise<string[]> => {
    return await invoke('stop')
}