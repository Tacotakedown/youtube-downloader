import { type FC } from 'react'

type T_ButtonProps = {
  text: string
  onClick: () => void
}

export const Button: FC<T_ButtonProps> = (props: T_ButtonProps): JSX.Element => {
  return (
    <button
      className="text-white  focus:outline-none   font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center bg-blue-600 hover:bg-blue-700 "
      onClick={props.onClick}
    >
      {props.text}
    </button>
  )
}
