import '@std/dotenv/load'
import { parseArgs, ParseOptions } from '@std/cli/parse-args'
import { promptSecret } from '@std/cli/prompt-secret'

import meta from './deno.json' with { type: 'json' }
import { ping, upload, version } from './fetchServer.ts'

function printUsage() {
  console.log('Usage: ')
  console.log('Ping to backup server:')
  console.log('  client --ping')
  console.log('Show the version number of backup server:')
  console.log('  client --version-server')
  console.log('Upload backup file:')
  console.log('  client --upload <file>')
  console.log('Options:')
  console.log('  -h, --help               Show this help message')
  console.log('  -v, --version            Show the version number')
  console.log('  -p, --ping               Ping the backup server')
  console.log(
    '  -s, --version-server     Show the version number of backup server',
  )
  console.log('  -u, --upload             Upload backup file')
}

const options: ParseOptions = {
  boolean: ['help', 'version'],
  string: ['URL', 'token', 'file'],
  default: { URL: 'http://0.0.0.0:8080', token: 'xxx', file: '' },
  alias: {
    help: 'h',
    version: 'v',
    ping: 'p',
    'version-server': 's',
    upload: 'u',
  },
}
const args = parseArgs(Deno.args, options)

if (args.help) {
  printUsage()
  Deno.exit(0)
} else if (args.version) {
  // Pro tip: add a version to your deno.json file
  console.log(meta.version)
  Deno.exit(0)
}

// validate the input and output arguments
if (!args.URL || !args.token) {
  console.log('You must specify both a URL and a token')
  printUsage()
  Deno.exit(1)
}

// attempt to get the username and password from environment variables
let urlBackupServer = Deno.env.get('URL_BACKUP_SERVER')
let tokenSecret = Deno.env.get('TOKEN_SECRET')

if (urlBackupServer === undefined) {
  const urlPrompt = prompt('Please enter the backup server URL:')
  urlBackupServer = urlPrompt ?? ''
}
if (tokenSecret === undefined) {
  const token = await promptSecret('Please enter the token secret:')
  tokenSecret = token ?? ''
}

if (args.ping) {
  const [isAlive, message] = await ping(urlBackupServer)
  if (isAlive) {
    console.log('Server responded: ' + message)
  } else {
    console.log(message)
  }
} else if (args['version-server']) {
  const [success, versionInfo] = await version(urlBackupServer, tokenSecret)
  if (success && typeof versionInfo === 'object') {
    console.log('Backup Server Version Information:')
    console.log(`Name: ${versionInfo.name}`)
    console.log(`Author: ${versionInfo.author}`)
    console.log(`Version: ${versionInfo.version}`)
    console.log(`Database Version: ${versionInfo.versionDatabase}`)
    console.log(`OS Version: ${versionInfo.versionOs}`)
    console.log(`Runtime Version: ${versionInfo.versionRuntime}`)
  } else {
    console.log(versionInfo)
  }
} else if (args.upload) {
  const file = args.upload
  const [success, message] = await upload(file, urlBackupServer, tokenSecret)
  if (success) {
    console.log(message)
  } else {
    const errorMessage = typeof message === 'string' ? message : (message.message || 'Unknown error')
    console.log(`Upload failed: ${errorMessage}`)
  }
} else {
  console.log('No valid command provided.')
  printUsage()
  Deno.exit(1)
}
