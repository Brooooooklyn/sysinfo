import { cpu } from 'systeminformation'

import { SysInfo } from './index.js'

const sys = new SysInfo()

console.time('systeminformation')
const cpuInfo = await cpu()
console.info(`CPU ${cpuInfo.manufacturer} ${cpuInfo.brand} frequency: ${cpuInfo.speed} MHz`)
console.timeEnd('systeminformation')

console.time('SysInfo')
for (const cpu of sys.cpus()) {
  console.info(`CPU ${cpu.name()} frequency: ${cpu.frequency()} MHz`)
}
console.timeEnd('SysInfo')
