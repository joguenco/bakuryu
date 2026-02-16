import '@std/dotenv/load'
import { parseArgs, ParseOptions } from '@std/cli/parse-args'
import { promptSecret } from '@std/cli/prompt-secret'

import meta from './deno.json' with { type: 'json' }
import { simulateTask } from './taskRunner.ts'

function printUsage() {
  console.log('Usage: ')
  console.log('  client --input <input file> --output <output file>')
  console.log('Options:')
  console.log('  -h, --help        Show this help message')
  console.log('  -v, --version     Show the version number')
  console.log('  -i, --input       Input file')
  console.log('  -o, --output      Output file')
}

const options: ParseOptions = {
  boolean: ['help', 'version'],
  string: ['URL', 'token'],
  default: { 'URL': 'https://example.com/data', 'token': 'xxx' },
  alias: { 'help': 'h', 'version': 'v', 'input': 'i', 'output': 'o' },
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

if (user === undefined) {
  const userPrompt = prompt('Please enter the username:')
  user = userPrompt ?? ''
}
if (password === undefined) {
  const passPrompt = promptSecret('Please enter the password:')
  password = passPrompt ?? ''
}

// simulating a few long-running tasks
await simulateTask(1000, `Reading input file [${args.URL}]`)
await simulateTask(1500, `Connecting with user [${user}]`)
await simulateTask(5000, `Reading data from external system`)
await simulateTask(3200, `Writing output file [${args.token}]`)

console.log('Done!')
