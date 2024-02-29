import { invoke } from '@tauri-apps/api/tauri'

export const downloadVideo = (id: string, path: string, format: string): void => {
  invoke('download_youtube_video', { format: format, path: path, id: id }).catch((e) => {
    console.error(e)
  })
}
