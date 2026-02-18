import '@std/dotenv/load'
import { parseArgs, ParseOptions } from '@std/cli/parse-args'
import { promptSecret } from '@std/cli/prompt-secret'

import meta from './deno.json' with { type: 'json' }
import { ping } from './fecthServer.ts'

function printUsage() {
  console.log('Usage: ')
  console.log('  client --input <input file> --output <output file>')
  console.log('Options:')
  console.log('  -h, --help        Show this help message')
  console.log('  -v, --version     Show the version number')
  console.log('  -p, --ping        Ping the backup server')
  console.log('  -i, --input       Input file')
  console.log('  -o, --output      Output file')
}

const options: ParseOptions = {
  boolean: ['help', 'version'],
  string: ['URL', 'token'],
  default: { 'URL': 'https://example.com/data', 'token': 'xxx' },
  alias: { 'help': 'h', 'version': 'v', 'input': 'i', 'output': 'o', 'ping': 'p' },
}
const args = parseArgs(Deno.args, options)

if (args.help) {
  printUsage()
  Deno.exit(0)
} else if (args.version) {
  // Pro tip: add a version to your deno.json file
  console.log(meta.version ? meta.version : '1.0.0')
  Deno.exit(0)
}

// validate the input and output arguments
if (!args.URL || !args.token) {
  console.log('You must specify both a URL and a token')
  printUsage()
  Deno.exit(1)
}

// attempt to get the username and password from environment variables
let user = Deno.env.get('MY_APP_USER')
let password = Deno.env.get('MY_APP_PASSWORD')
let urlBackupServer = Deno.env.get('URL_BACKUP_SERVER')

if (user === undefined) {
  const userPrompt = prompt('Please enter the username:')
  user = userPrompt ?? ''
}
if (password === undefined) {
  const passPrompt = promptSecret('Please enter the password:')
  password = passPrompt ?? ''
}
if (urlBackupServer === undefined) {
  const urlPrompt = prompt('Please enter the backup server URL:')
  urlBackupServer = urlPrompt ?? ''
}

if (args.ping) {
  const [isAlive, message] = await ping(urlBackupServer)
  if (isAlive) {
    console.log('Server responded: ' + message)
  } else {
    console.log(message)
  }
}
