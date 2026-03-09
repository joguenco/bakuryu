import { Spinner } from '@std/cli/unstable-spinner'
import { ErrorMessage } from './errorMessage.ts'

interface PingResponse {
  message: string
}

interface VersionResponse {
  name: string
  author: string
  version: string
  versionDatabase: string
  versionOs: string
  versionRuntime: string
}

export async function upload(
  file: string,
  urlBackupServer: string,
  token: string,
): Promise<[boolean, ErrorMessage | string]> {
  const path = urlBackupServer + '/backup'
  const message = `Uploading ...`
  const spinner = new Spinner({ message, color: 'yellow' })
  let statusResponse = false
  let uploadMessage: ErrorMessage | string = ''
  const start = performance.now()

  try {
    spinner.start()
    const fileData = await Deno.readFile(file)
    const hashBuffer = await crypto.subtle.digest('SHA-256', fileData)
    const sha256 = Array.from(new Uint8Array(hashBuffer)).map((b) => b.toString(16).padStart(2, '0')).join('')
    const formData = new FormData()
    formData.append('sha2', sha256)
    let fileName = 'file'

    const os = Deno.build.os
    if (os === 'windows') {
      fileName = file.split('\\').pop() || 'file'
    } else if (os === 'linux' || os === 'darwin') {
      fileName = file.split('/').pop() || 'file'
    } else {
      fileName = 'file'
    }

    formData.append('file', new Blob([fileData]), fileName)
    const response = await fetch(path, {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${token}` },
      body: formData,
    })

    if (response.ok) {
      statusResponse = true
      uploadMessage = 'File uploaded successfully'
    } else {
      uploadMessage = await response.json() as ErrorMessage
    }
  } catch (error) {
    statusResponse = false
    console.error('Error during file upload:', error)
  } finally {
    spinner.stop()
    const finish = performance.now()
    const duration = Math.round((finish - start) / 100) / 10
    console.log(`${message} (${duration.toFixed(1)}s).`)
  }
  return [statusResponse, uploadMessage]
}

export async function version(urlBackupServer: string, token: string): Promise<[boolean, VersionResponse | string]> {
  const path = urlBackupServer + '/version'
  const message = `Retrieving version information from backup server ${urlBackupServer}`
  const spinner = new Spinner({ message, color: 'yellow' })
  let statusResponse = false
  let versionResponse: VersionResponse | string = ''
  const start = performance.now()

  try {
    spinner.start()
    await sleep(900)
    const response = await fetch(path, { headers: { 'Authorization': `Bearer ${token}` } })

    versionResponse = await response.json() as VersionResponse
    statusResponse = response.ok
  } catch (_) {
    versionResponse = 'Failed to retrieve version'
  } finally {
    spinner.stop()
    const finish = performance.now()
    const duration = Math.round((finish - start) / 100) / 10
    console.log(`${message} (${duration.toFixed(1)}s).`)
  }
  return [statusResponse, versionResponse]
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
