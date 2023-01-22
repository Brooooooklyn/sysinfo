import test from 'ava'

import { cpuFeatures, SysInfo } from '../index.js'

test('cpuFeatures', (t) => {
  t.notThrows(() => cpuFeatures())
})

test('SysInfo', (t) => {
  const sysinfo = new SysInfo()
  t.notThrows(() => sysinfo.refreshMemory())
  for (const cpu of sysinfo.cpus()) {
    console.info(`CPU ${cpu.name()} frequency: ${cpu.frequency()} MHz`)
    t.is(typeof cpu.frequency(), 'number')
  }
})
