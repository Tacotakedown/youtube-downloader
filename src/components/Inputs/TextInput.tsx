import { type ChangeEvent, type FC } from 'react'

type T_TextInputProps = {
  value: string
  onChange: (value: string) => void
  placeholderText: string
}

export const TextInput: FC<T_TextInputProps> = (props: T_TextInputProps): JSX.Element => {
  const handleInputChange = (event: ChangeEvent<HTMLInputElement>): void => {
    props.onChange(event.target.value)
  }

  return (
    <input
      className="text-sm rounded-lg  block w-full p-2.5 bg-black/55 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
      type="text"
      value={props.value}
      onChange={handleInputChange}
      placeholder={props.placeholderText}
    />
  )
}
