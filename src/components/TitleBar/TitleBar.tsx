import { type FC } from 'react'
import { appWindow } from '@tauri-apps/api/window'
import { Icon } from '../Icon/Icon'

type T_TitleBarProps = {
  title: string
}

export const TitleBar: FC<T_TitleBarProps> = (props: T_TitleBarProps): JSX.Element => {
  return (
    <div
      data-tauri-drag-region
      style={{ userSelect: 'none', zIndex: 10000 }}
      className="drag w-screen h-9  bg-black/70 px-1 flex flex-row-reverse justify-between items-center text-slate-400 "
    >
      <div className="no-drag flex justify-end">
        <Icon
          name="minus"
          className="hover:text-gray-300 duration-200"
          onClick={() => {
            appWindow.minimize().catch((e) => {
              console.error(e)
            })
          }}
        />
        <Icon
          name="close"
          className="hover:text-red-500 duration-200"
          onClick={() => {
            appWindow.close().catch((e) => {
              console.error(e)
            })
          }}
        />
      </div>
      <span data-tauri-drag-region>{props.title}</span>
      <div className="no-drag flex justify-center gap-1">
        <Icon name="cog" className="hover:text-gray-300 duration-200" />
      </div>
    </div>
  )
}
