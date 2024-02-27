import { type FC, useEffect, useState } from 'react'

type T_QueueProps = {
  queue: string[]
  handleRemove: (id: string) => void
}

export const Queue: FC<T_QueueProps> = (props: T_QueueProps): JSX.Element => {
  const [localQueue, setLocalQueue] = useState<string[]>([])
  useEffect(() => {
    setLocalQueue(props.queue)
  }, [props.queue])
  const renderQueue = (): JSX.Element[] => {
    return localQueue.map((queue: string, key: number) => <div key={key}>{queue}</div>)
  }

  return <div className="flex flex-col">Queue:{renderQueue()}</div>
}
