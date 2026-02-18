import { Spinner } from '@std/cli/unstable-spinner'

interface PingResponse {
  message: string
}

export async function ping(urlBackupServer: string): Promise<[boolean, string]> {
  const path = urlBackupServer + '/ping'
  const message = `Pinging backup server at ${urlBackupServer}`
  const spinner = new Spinner({ message, color: 'yellow' })
  let statusResponse = false
  let pingMessage = ''
  const start = performance.now()

  try {
    spinner.start()
    await sleep(900)
    const response = await fetch(path, { method: 'GET' })
    const pingResponse = await response.json() as PingResponse

    statusResponse = response.ok
    pingMessage = pingResponse.message
  } catch (_) {
    statusResponse = false
    pingMessage = 'Server is not available'
  } finally {
    spinner.stop()
    const finish = performance.now()
    const duration = Math.round((finish - start) / 100) / 10
    console.log(`${message} (${duration.toFixed(1)}s).`)
  }

  return [statusResponse, pingMessage]
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
