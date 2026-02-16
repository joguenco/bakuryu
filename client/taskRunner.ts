import { Spinner } from '@std/cli/unstable-spinner'

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export async function simulateTask(ms: number, message: string) {
  const spinner = new Spinner({ message, color: 'yellow' })
  const start = performance.now()
  spinner.start()
  await sleep(ms)
  spinner.stop()
  const finish = performance.now()
  const duration = Math.round((finish - start) / 100) / 10
  console.log(`${message} (${duration.toFixed(1)}s).`)
}
