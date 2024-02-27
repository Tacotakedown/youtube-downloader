import { useState } from 'react'
import { TextInput } from '../../components/Inputs/TextInput'
import { TextGenerateEffect } from '../../components/ui/TextGenerateEffect/TextGenerateEffect'
import { Button } from '../../components/Inputs/Button'
import { SelectionInput } from '../../components/Inputs/SelectionInput'
import { Queue } from '../../components/Queue/Queue'
import { toast } from 'react-toastify'

export const Home = (): JSX.Element => {
  const [textValue, setTextValue] = useState<string>('')
  const handleTextChange = (value: string): void => {
    setTextValue(value)
  }
  const [selectionValue, setSelectionValue] = useState<string>('')
  const handleSelcetionChange = (value: string): void => {
    setSelectionValue(value)
  }
  const [queue, setQueue] = useState<string[]>([])
  const handleAddToQueue = (value: string): void => {
    const newQueue = [...queue]
    if (newQueue.includes(value)) {
      toast(`${value} is already in the queue`)
      return
    }
    newQueue.push(value)
    setQueue(newQueue)
  }
  return (
    <div>
      <TextGenerateEffect words="Welcome to YouTube Downloader" />
      <TextInput value={textValue} onChange={handleTextChange} placeholderText="Paste Link here" />
      <Button text="Paste" onClick={() => {}} />
      <SelectionInput options={['MP3', 'WAV', 'MP4', 'MKV']} value={selectionValue} onChange={handleSelcetionChange} />
      <Button
        text="Add to Queue"
        onClick={() => {
          handleAddToQueue(textValue)
        }}
      />
      <Queue queue={queue} handleRemove={() => {}} />
    </div>
  )
}
