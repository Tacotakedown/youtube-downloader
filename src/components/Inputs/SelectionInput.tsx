import { type FC, type ChangeEvent } from 'react'

type T_SelectionInputProps = {
  options: string[]
  value: string
  onChange: (value: string) => void
}

export const SelectionInput: FC<T_SelectionInputProps> = (props: T_SelectionInputProps): JSX.Element => {
  const handleSelctionChange = (event: ChangeEvent<HTMLSelectElement>): void => {
    props.onChange(event.target.value)
  }

  return (
    <select value={props.value} onChange={handleSelctionChange}>
      {props.options.map((option, index) => (
        <option key={index} value={option}>
          {option}
        </option>
      ))}
    </select>
  )
}
