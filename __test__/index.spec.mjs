import test from "ava"

import { cpuFeatures, SysInfo } from "../index.js"

test("cpuFeatures", (t) => {
  const { arch } = cpuFeatures()
  console.info(`CPU architecture: ${arch}`)
  t.is(typeof arch, "string")
})

test("SysInfo", (t) => {
  const sysinfo = new SysInfo()
  t.notThrows(() => sysinfo.refreshMemory())
  for (const cpu of sysinfo.cpus()) {
    console.info(`CPU ${cpu.name()} frequency: ${cpu.frequency()} MHz`)
    t.is(typeof cpu.frequency(), "number")
  }
})

test("Process functionality", (t) => {
  const sysinfo = new SysInfo()

  t.notThrows(() => sysinfo.refreshProcesses())

  const processes = sysinfo.processes()
  t.true(Array.isArray(processes))
  t.true(processes.length > 0)
  console.info(`Found ${processes.length} processes`)

  for (let i = 0; i < Math.min(3, processes.length); i++) {
    const proc = processes[i]
    t.is(typeof proc.pid(), "number")
    t.is(typeof proc.name(), "string")
    t.is(typeof proc.cpuUsage(), "number")
    t.is(typeof proc.memory(), "bigint")
    console.info(
      `PID: ${proc.pid()}, Name: ${proc.name()}, CPU: ${proc.cpuUsage()}%, Memory: ${proc.memory()} bytes`
    )
  }
})

test("Current process details", (t) => {
  const sysinfo = new SysInfo()
  sysinfo.refreshProcesses()

  const currentPid = process.pid
  const currentProcess = sysinfo.processByPid(currentPid)

  t.truthy(currentProcess)

  if (currentProcess) {
    t.is(currentProcess.pid(), currentPid)
    t.is(typeof currentProcess.name(), "string")
    t.is(typeof currentProcess.memory(), "bigint")
    t.is(typeof currentProcess.virtualMemory(), "bigint")
    t.is(typeof currentProcess.cpuUsage(), "number")
    t.is(typeof currentProcess.startTime(), "bigint")
    t.is(typeof currentProcess.runTime(), "bigint")

    const exe = currentProcess.exe()
    if (exe !== null) {
      t.is(typeof exe, "string")
    }

    const cwd = currentProcess.cwd()
    if (cwd !== null) {
      t.is(typeof cwd, "string")
    }

    const parent = currentProcess.parent()
    if (parent !== null) {
      t.is(typeof parent, "number")
    }

    const cmd = currentProcess.cmd()
    t.true(Array.isArray(cmd))

    const environ = currentProcess.environ()
    t.is(typeof environ, "object")

    const diskUsage = currentProcess.diskUsage()
    t.is(typeof diskUsage.readBytes, "number")
    t.is(typeof diskUsage.writtenBytes, "number")
    t.is(typeof diskUsage.totalReadBytes, "number")
    t.is(typeof diskUsage.totalWrittenBytes, "number")

    console.info(
      `CurrentProcess: ${currentProcess.name()} (PID: ${currentProcess.pid()})`
    )
    console.info(
      `Mem: ${(Number(currentProcess.memory()) / 1024 / 1024).toFixed(2)} MB`
    )
    console.info(`Executable: ${currentProcess.exe()}`)
  }
})

test("Process search by name", (t) => {
  const sysinfo = new SysInfo()
  const refreshedCount = sysinfo.refreshProcesses(true)
  t.is(typeof refreshedCount, 'number')
  t.true(refreshedCount > 0)

  const nodeProcesses = sysinfo.processesByName("node")
  t.true(Array.isArray(nodeProcesses))
  console.info(`Found ${nodeProcesses.length} node processes`)

  if (nodeProcesses.length > 0) {
    const nodeProc = nodeProcesses[0]
    t.is(typeof nodeProc.pid(), "number")
    t.is(typeof nodeProc.name(), "string")
    t.true(nodeProc.name().toLowerCase().includes("node"))
    console.info(`Node process PID: ${nodeProc.pid()}`)
  }
})

test("Process refresh by PIDs", (t) => {
  const sysinfo = new SysInfo()
  const initialCount = sysinfo.refreshProcesses(true)
  t.is(typeof initialCount, 'number')

  const someProcesses = sysinfo.processes().slice(0, 5)
  const pids = someProcesses.map((p) => p.pid())

  t.notThrows(() => {
    const updatedCount = sysinfo.refreshProcessesSpecifics(pids, true)
    t.is(typeof updatedCount, 'number')
    console.info(`Updated ${updatedCount} processes for PIDs: ${pids.join(", ")} (default config)`)
  })

  t.notThrows(() => {
    const refreshConfig = {
      memory: true,
      cpu: true,
      disk_usage: false,
      exe: true,
      cmd: false,
      environ: false,
      cwd: false,
      user: false,
      tasks: false
    }
    const updatedCount = sysinfo.refreshProcessesSpecifics(pids, true, refreshConfig)
    t.is(typeof updatedCount, 'number')
    console.info(`Updated ${updatedCount} processes with custom config`)
  })
})
